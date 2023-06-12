use super::{nonce::Nonce, public_key_hash::PublicKeyHash};

/// Rperesents the error of the read_input functions
#[derive(Debug)]
pub enum ReadInputError {
    /// The message does not be process by this rollup
    NotATzwitterMessage,
    /// There is no more messages
    EndOfInbox,
    /// There is an error in the bytes to string deserialization
    FromUtf8Error(std::string::FromUtf8Error),
    /// There is an error in the string to Message deserialization
    SerdeJson(serde_json_wasm::de::Error),
    /// There is an error runtime
    Runtime(tezos_smart_rollup::host::RuntimeError),
}

/// Represents all the error of the kernel
///
#[derive(Debug)]
pub enum Error {
    FromUtf8(std::string::FromUtf8Error),
    Runtime(tezos_smart_rollup::host::RuntimeError),
    Ed25519Compact(ed25519_compact::Error),
    InvalidSignature,
    InvalidNonce {
        current_nonce: Nonce,
        given_nonce: Nonce,
    },
    PathError(tezos_smart_rollup::storage::path::PathError),
    StateDeserializarion,
    TweetNotFound {
        tweet_id: u64,
    },
    TweetAlreadyLiked {
        tweet_id: u64,
    },
    NotOwner {
        tweet_id: u64,
        address: PublicKeyHash,
    },
    TweetAlreadyCollected {
        tweet_id: u64,
    },
    FromBase58CheckError,
    BigIntError,
    BinError(tezos_data_encoding::enc::BinError),
    EntrypointError(tezos_smart_rollup::types::EntrypointError),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::FromUtf8(_) => "Cannot convert bytes to string".into(),
            Error::Runtime(_) => "Runtime error, caused by host function".into(),
            Error::Ed25519Compact(_) => "Cannot deserialize Ed25519".into(),
            Error::InvalidSignature => "Invalid signature".into(),
            Error::InvalidNonce {
                current_nonce,
                given_nonce,
            } => format!(
                "Invalid nonce, current: {}, given: {}",
                current_nonce.0, given_nonce.0
            ),
            Error::PathError(_) => "Invalid path".into(),
            Error::StateDeserializarion => "State deserialization".into(),
            Error::TweetNotFound { tweet_id } => format!("Tweet {} not found", tweet_id),
            Error::TweetAlreadyLiked { tweet_id } => {
                format!(
                    "The tweet {} has already been liked by this account",
                    tweet_id
                )
            }
            Error::NotOwner { tweet_id, address } => {
                format!(
                    "{} is not the owner of the tweet {}",
                    address.to_string(),
                    tweet_id
                )
            }
            Error::TweetAlreadyCollected { tweet_id } => {
                format!("The tweet {} has already been collected", tweet_id)
            }
            Error::FromBase58CheckError => "Cannot convert a string to a contract address".into(),
            Error::BigIntError => "Cannot deserialize big int".into(),
            Error::BinError(_) => "Cannot serialize michelson to binary".into(),
            Error::EntrypointError(_) => "Not a correct entrypoint".into(),
        }
    }
}

macro_rules! register_error {
    ($name:ident, $error:ty) => {
        impl From<$error> for Error {
            fn from(data: $error) -> Self {
                Error::$name(data)
            }
        }
    };
}

register_error!(FromUtf8, std::string::FromUtf8Error);
register_error!(Ed25519Compact, ed25519_compact::Error);
register_error!(PathError, tezos_smart_rollup::storage::path::PathError);
register_error!(Runtime, tezos_smart_rollup::host::RuntimeError);
register_error!(BinError, tezos_data_encoding::enc::BinError);
register_error!(EntrypointError, tezos_smart_rollup::types::EntrypointError);

pub type Result<A> = std::result::Result<A, Error>;
