use super::{hex_bytes_32, hex_bytes_64};

use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::Type;

// Deserialize from hex, 64-characters -> 32-bytes
// Serialize into hex, 32-bytes -> 128-characters
#[derive(Clone, Copy, Debug, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct Bytes32(#[serde(with = "hex_bytes_32")] pub [u8; 32]);

// Deserialize from hex, 128-characters -> 64-bytes
// Serialize into hex, 64-bytes -> 128-characters
#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct Bytes64(#[serde(with = "hex_bytes_64")] pub [u8; 64]);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct IkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct IkPubEd(pub Bytes32);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct SpkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct SpkPubSig(pub Bytes64);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct SigData(pub Bytes64);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct OtpkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Nonce(pub Bytes32);

impl Nonce {
    pub fn generate() -> Self {
        let random_bytes: [u8; 32] = random();
        Self(Bytes32(random_bytes))
    }
}
