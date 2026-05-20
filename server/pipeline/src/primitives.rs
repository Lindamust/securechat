use rand::random;
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use sqlx::Type;
use std::{
    convert::{AsRef, TryFrom},
    str::FromStr,
};

use crate::error::PipelineError;

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

impl TryFrom<&[u8]> for Bytes32 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl TryFrom<Vec<u8>> for Bytes32 {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl AsRef<[u8]> for Bytes32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromStr for Bytes32 {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer()).map_err(|_: serde::de::value::Error| {
            PipelineError::Internal("Bytes32 FromStr fail".to_owned())
        })
    }
}

impl TryFrom<&[u8]> for Bytes64 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl TryFrom<Vec<u8>> for Bytes64 {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl AsRef<[u8]> for Bytes64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

macro_rules! impl_from_bytes {
    ($t:ty, $inner:ty) => {
        impl From<Vec<u8>> for $t {
            fn from(v: Vec<u8>) -> Self {
                <$inner>::try_from(v).map(Self).expect(concat!(
                    stringify!($t),
                    ": invalid byte length from database"
                ))
            }
        }
    };
}

impl_from_bytes!(IkPub, Bytes32);
impl_from_bytes!(IkPubEd, Bytes32);
impl_from_bytes!(SpkPub, Bytes32);
impl_from_bytes!(SpkPubSig, Bytes64);
impl_from_bytes!(SigData, Bytes64);
impl_from_bytes!(OtpkPub, Bytes32);
impl_from_bytes!(Nonce, Bytes32);

pub mod hex_bytes {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<const N: usize, S>(bytes: &[u8; N], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, const N: usize, D>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut bytes = [0u8; N];

        hex::decode_to_slice(&s, &mut bytes)
            .map_err(|e| D::Error::custom(format!("Hex decode error: {e}")))?;

        Ok(bytes)
    }
}

macro_rules! hex_bytes_module {
    ($mod_name:ident, $n:expr) => {
        pub mod $mod_name {
            use super::hex_bytes;
            use serde::{Deserializer, Serializer};

            pub fn serialize<S>(bytes: &[u8; $n], s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                hex_bytes::serialize::<$n, _>(bytes, s)
            }

            pub fn deserialize<'de, D>(d: D) -> Result<[u8; $n], D::Error>
            where
                D: Deserializer<'de>,
            {
                hex_bytes::deserialize::<$n, _>(d)
            }
        }
    };
}

hex_bytes_module!(hex_bytes_32, 32);
hex_bytes_module!(hex_bytes_64, 64);
