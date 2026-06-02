use primitives::{Bytes32, Bytes64, ReprBytes, impl_from_vec, impl_repr_bytes};
use serde::{Deserialize, Serialize};
use sqlx::Type;

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

#[derive(Serialize, Deserialize, Debug, Type, Clone)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct NonceKey(pub Bytes32);

impl NonceKey {
    pub fn generate() -> Self {
        let random_bytes: [u8; 32] = rand::random();
        Self(Bytes32(random_bytes))
    }
}

impl_repr_bytes!(
    IkPub<32>,
    IkPubEd<32>,
    SpkPub<32>,
    SpkPubSig<64>,
    SigData<64>,
    OtpkPub<32>,
    NonceKey<32>,
);

impl_from_vec!(
    (IkPub, Bytes32),
    (IkPubEd, Bytes32),
    (SpkPub, Bytes32),
    (SpkPubSig, Bytes64),
    (SigData, Bytes64),
    (OtpkPub, Bytes32),
    (NonceKey, Bytes32),
);
