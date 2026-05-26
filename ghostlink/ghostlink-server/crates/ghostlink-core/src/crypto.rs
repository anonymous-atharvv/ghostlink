use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

/// Signal Protocol pre-key (one-time use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneTimePreKey {
    pub key_id: i32,
    pub public_key: String, // Base64-encoded
}

/// Signal Protocol signed pre-key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPreKey {
    pub key_id: i32,
    pub public_key: String, // Base64-encoded
    pub signature: String,  // Base64-encoded
}

/// Complete key bundle for X3DH key exchange.
/// Fetched by other users to establish encrypted sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBundle {
    pub account_id: Uuid,
    pub identity_key: String,
    pub signed_pre_key: SignedPreKey,
    pub one_time_pre_key: Option<OneTimePreKey>,
}

/// Key material container — zeroized on drop.
/// Used internally when handling private key operations.
#[derive(Debug, Zeroize)]
#[zeroize(drop)]
pub struct SensitiveKeyMaterial {
    pub bytes: Vec<u8>,
}

impl SensitiveKeyMaterial {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
