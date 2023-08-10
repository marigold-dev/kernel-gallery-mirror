// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

use crate::{
    constants::{L1_TOKEN_CONTRACT_ADDRESS, L1_TOKEN_CONTRACT_ENTRYPOINT, MAGIC_BYTE},
    core::{
        account::Account,
        message::{Content, Inner, PostImage, PostTweet, Transfer},
        nonce::Nonce,
        tweet::{Tweet, TweetContent},
    },
    get_dac_committee,
    storage::{
        self, add_collecting_tweet_to_account, add_owned_tweet_to_account,
        add_written_tweet_to_account, increment_tweet_counter, is_liked, is_not_collected,
        is_owner, read_tweet, set_collected_block, set_like_flag, store_tweet,
    },
};

use num_bigint::ToBigInt;
use tezos_data_encoding::{enc::BinWriter, types::Zarith};
use tezos_smart_rollup::{
    michelson::{MichelsonContract, MichelsonInt, MichelsonPair, MichelsonString},
    outbox::{OutboxMessage, OutboxMessageTransaction, OutboxMessageTransactionBatch},
    prelude::*,
    storage::path::RefPath,
    types::{Contract, Entrypoint},
};

use crate::core::error::*;
use crate::core::message::Message;

/// Read a message from the inbox
///
/// It will only read messages External Messages with the MAGIC_BYTE
/// Benchmark: 2_000_000 ticks (processing an inbox with only one message)
pub fn read_input<R: Runtime>(host: &mut R) -> std::result::Result<(Message, u32), ReadInputError> {
    let input = host.read_input().map_err(ReadInputError::Runtime)?;
    match input {
        None => Err(ReadInputError::EndOfInbox),
        Some(message) => {
            let data = message.as_ref();
            match data {
                [0x01, MAGIC_BYTE, ..] => {
                    let bytes = data.iter().skip(2).copied().collect();
                    let str = String::from_utf8(bytes).map_err(ReadInputError::FromUtf8Error)?;
                    let msg = serde_json_wasm::from_str(&str).map_err(ReadInputError::SerdeJson)?;
                    Ok((msg, message.level))
                }
                _ => Err(ReadInputError::NotATzwitterMessage),
            }
        }
    }
}

/// Verify the signature of a message
///
/// Returns the inner message
pub fn verify_signature(message: Message) -> Result<Inner> {
    let signature = message.signature();
    let pkey = message.public_key();
    let inner = message.inner();
    let hash = inner.hash();

    signature.verify(pkey, hash.as_ref())?;
    let Message { inner, .. } = message;
    Ok(inner)
}

/// Verify the nonce of the inner message
///
/// If the nonce is correct the content of the inner is returned
pub fn verify_nonce(inner: Inner, nonce: &Nonce) -> Result<Content> {
    let next_nonce = nonce.next();
    let inner_nonce = inner.nonce();
    if &next_nonce == inner_nonce {
        let Inner { content, .. } = inner;
        Ok(content)
    } else {
        Err(Error::InvalidNonce)
    }
}

/// Create a new image tweet from the PostImage request
/// Save the tweet_image to the durable state
/// And add a tweet entry to the user account
pub fn create_image<R: Runtime>(
    host: &mut R,
    account: &Account,
    post_image: PostImage,
) -> Result<()> {
    let certificate = post_image.content;
    let committee_members_pks = get_dac_committee()?;
    certificate
        .verify(&committee_members_pks, 2)
        .map_err(Error::from)?;
    let path: RefPath = RefPath::assert_from(b"/reveal/image");
    certificate
        .reveal_to_store(host, &path)
        .map_err(Error::from)?;
    let bytes = host.store_read_all(&path).map_err(Error::from)?;
    let tweet = Tweet {
        author: post_image.author,
        content: TweetContent::Image(bytes),
        likes: 0,
    };
    let id = increment_tweet_counter(host)?;
    let _ = store_tweet(host, &id, &tweet)?;
    add_owned_tweet_to_account(host, &account.public_key_hash, &id)?;
    add_written_tweet_to_account(host, &account.public_key_hash, &id)?;
    Ok(())
}

/// Create a new tweet from the PostTweet request
/// Save the tweet to the durable state
/// And add a tweet entry to the user account
pub fn create_tweet<R: Runtime>(
    host: &mut R,
    account: &Account,
    post_tweet: PostTweet,
) -> Result<()> {
    let id = increment_tweet_counter(host)?;
    let tweet = Tweet::from(post_tweet);
    let _ = store_tweet(host, &id, &tweet)?;
    add_owned_tweet_to_account(host, &account.public_key_hash, &id)?;
    add_written_tweet_to_account(host, &account.public_key_hash, &id)?;
    Ok(())
}

pub fn like_tweet<R: Runtime>(host: &mut R, account: &Account, tweet_id: &u64) -> Result<()> {
    let already_liked = is_liked(host, &account.public_key_hash, tweet_id)?;
    match already_liked {
        true => Err(Error::TweetAlreadyLiked),
        false => {
            let tweet = read_tweet(host, tweet_id)?;
            match tweet {
                None => Err(Error::TweetNotFound),
                Some(tweet) => {
                    let tweet = tweet.like();
                    store_tweet(host, tweet_id, &tweet)?;
                    set_like_flag(host, &account.public_key_hash, tweet_id)?;
                    Ok(())
                }
            }
        }
    }
}

/// Transfer a tweet from an account to another one
///
/// Checks if the account parameter is owner of the tweet
pub fn transfer_tweet<R: Runtime>(
    host: &mut R,
    account: &Account,
    transfer: &Transfer,
) -> Result<()> {
    let Transfer {
        tweet_id,
        destination,
    } = transfer;
    is_owner(host, &account.public_key_hash, tweet_id)?;
    storage::transfer(host, &account.public_key_hash, tweet_id, destination)?;
    Ok(())
}

/// Withdraw the tweet to layer 1
pub fn withdraw_tweet<R: Runtime>(
    host: &mut R,
    level: u32,
    account: &Account,
    tweet_id: &u64,
) -> Result<()> {
    is_owner(host, &account.public_key_hash, tweet_id)?;
    is_not_collected(host, tweet_id)?;

    let tweet = read_tweet(host, tweet_id)
        .map_err(Error::from)?
        .ok_or(Error::TweetNotFound)?;

    let owner = {
        let contract = Contract::from_b58check(&account.public_key_hash.to_string())
            .map_err(|_| Error::FromBase58CheckError)?;
        MichelsonContract(contract)
    };
    let author = {
        let contract = Contract::from_b58check(&tweet.author.to_string())
            .map_err(|_| Error::FromBase58CheckError)?;
        MichelsonContract(contract)
    };
    let id = {
        let id = tweet_id.to_bigint().ok_or(Error::BigIntError)?;
        let id = Zarith(id);
        MichelsonInt(id)
    };
    // What to do with that?
    let likes = {
        let likes = tweet.likes.to_bigint().ok_or(Error::BigIntError)?;
        let likes = Zarith(likes);
        MichelsonInt(likes)
    };

    //let content = MichelsonString(tweet.content);
    let content = match &tweet.content {
        TweetContent::Image(image) => {
            let str = hex::encode(image);
            MichelsonString(str)
        }
        TweetContent::Text(text) => MichelsonString(text.clone()),
    };

    let destination = Contract::from_b58check(L1_TOKEN_CONTRACT_ADDRESS)
        .map_err(|_| Error::FromBase58CheckError)?;

    // (pair %mint
    //     (pair (nat %id) (address %owner))
    //     (pair %token (pair (address %author) (string %content)) (nat %likes)))

    let michelson = MichelsonPair(
        MichelsonPair(id, owner),
        MichelsonPair(MichelsonPair(author, content), likes),
    );

    let transaction = OutboxMessageTransaction {
        parameters: michelson,
        destination,
        entrypoint: Entrypoint::try_from(L1_TOKEN_CONTRACT_ENTRYPOINT.to_string())
            .map_err(Error::from)?,
    };

    let batch = OutboxMessageTransactionBatch::from(vec![transaction]);
    let message = OutboxMessage::AtomicTransactionBatch(batch);

    let mut output = Vec::default();
    message.bin_write(&mut output).unwrap();

    host.write_output(&output).unwrap();

    // Freeze the tweets
    set_collected_block(host, tweet_id, &level)?;
    // Indicates that the user is collecting the tweet
    add_collecting_tweet_to_account(host, &account.public_key_hash, tweet_id)?;
    Ok(())
}
