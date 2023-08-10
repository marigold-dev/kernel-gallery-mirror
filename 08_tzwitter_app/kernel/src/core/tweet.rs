// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use serde::Serialize;

use crate::core::public_key_hash::PublicKeyHash;

use super::message::PostTweet;

#[derive(Serialize)]
pub struct Tweet {
    pub author: PublicKeyHash,
    pub content: TweetContent,
    pub likes: u64,
}

#[derive(Serialize)]
pub enum TweetContent {
    Image(Vec<u8>),
    Text(String),
}

impl From<PostTweet> for Tweet {
    fn from(post_tweet: PostTweet) -> Self {
        let PostTweet { author, content } = post_tweet;
        let tweet_content = TweetContent::Text(content);
        Tweet {
            author,
            content: tweet_content,
            likes: 0,
        }
    }
}

impl Tweet {
    pub fn like(self) -> Self {
        Self {
            likes: self.likes + 1,
            ..self
        }
    }
}
