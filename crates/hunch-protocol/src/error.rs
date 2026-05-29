//! Protocol-level error type.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("missing required tag: {0}")]
    MissingTag(&'static str),

    #[error("malformed tag {tag}: {detail}")]
    MalformedTag { tag: &'static str, detail: String },

    #[error("invalid outcome: expected YES, NO, or INVALID; got {0:?}")]
    InvalidOutcome(String),

    #[error("invalid kind: expected {expected}, got {actual}")]
    KindMismatch { expected: u32, actual: u32 },

    #[error("invalid pubkey hex: {0}")]
    InvalidPubkey(String),

    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("hex decode error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("secp256k1 error: {0}")]
    Secp(#[from] secp256k1::Error),

    #[error("signing not implemented for this signer: {0}")]
    SignerNotImplemented(&'static str),
}
