// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

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
    InvalidNonce,
    PathError(tezos_smart_rollup::storage::path::PathError),
    StateDeserializarion,
    TweetNotFound,
    TweetAlreadyLiked,
    NotOwner,
    TweetAlreadyCollected,
    FromBase58CheckError,
    BigIntError,
    BinError(tezos_data_encoding::enc::BinError),
    EntrypointError(tezos_smart_rollup::types::EntrypointError),
    CertificateError(tezos_smart_rollup::dac::certificate::CertificateError),
    FromHexError(hex::FromHexError),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        let err = match self {
            Error::FromUtf8(_) => "Cannot convert bytes to string",
            Error::Runtime(_) => "Runtime error, caused by host function",
            Error::Ed25519Compact(_) => "Cannot deserialize Ed25519",
            Error::InvalidSignature => "Invalid signature",
            Error::InvalidNonce => "Invalid nonce",
            Error::PathError(_) => "Invalid path",
            Error::StateDeserializarion => "State deserialization",
            Error::TweetNotFound => "Tweet not found",
            Error::TweetAlreadyLiked => "The tweet has already been liked by this account",
            Error::NotOwner => "Not the owner of the tweet",
            Error::TweetAlreadyCollected => "The tweet has already been collected",
            Error::FromBase58CheckError => "Cannot convert a string to a contract address",
            Error::BigIntError => "Cannot deserialize big int",
            Error::BinError(_) => "Cannot serialize michelson to binary",
            Error::EntrypointError(_) => "Not a correct entrypoint",
            Error::CertificateError(_) => "Cannot verify signature from certificate",
            Error::FromHexError(_) => "Cannot decode hexadecimal string to bytes",
        };
        err.to_string()
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
register_error!(
    CertificateError,
    tezos_smart_rollup::dac::certificate::CertificateError
);
register_error!(FromHexError, hex::FromHexError);

pub type Result<A> = std::result::Result<A, Error>;
