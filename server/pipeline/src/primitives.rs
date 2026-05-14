use rand::random;
use serde::{Deserialize, Serialize};

// Deserialize from hex, 64-characters -> 32-bytes
// Serialize into hex, 32-bytes -> 128-characters
#[derive(Clone, Copy, Debug)]
pub struct Bytes32(pub [u8; 32]);

// Deserialize from hex, 128-characters -> 64-bytes
// Serialize into hex, 64-bytes -> 128-characters
// manually implement because serde lacks them :(
#[derive(Debug)]
pub struct Bytes64(pub [u8; 64]);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct IkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug)]
pub struct IkPubEd(pub Bytes32);

#[derive(Serialize, Deserialize, Debug)]
pub struct SpkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug)]
pub struct SpkPubSig(pub Bytes64);

#[derive(Serialize, Deserialize, Debug)]
pub struct SigData(pub Bytes64);

#[derive(Serialize, Deserialize, Debug)]
pub struct OtpkPub(pub Bytes32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Nonce(pub Bytes32);

impl Nonce {
    pub fn generate() -> Self {
        let random_bytes: [u8; 32] = random();
        Self(Bytes32(random_bytes))
    }
}



/// IMPLS

use hex::{decode_to_slice, encode};
use serde::{Deserializer, Serializer, de::Visitor};

impl Serialize for Bytes32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = encode(self.0);
        serializer.serialize_str(&hex_string)
    }
}

impl Serialize for Bytes64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = encode(self.0);
        serializer.serialize_str(&hex_string)
    }
}

struct Bytes32HexVisitor;
struct Bytes64HexVisitor;

impl<'de> Visitor<'de> for Bytes32HexVisitor {
    type Value = Bytes32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 64-character hex string for a 32 byte array")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut bytes = [0u8; 32];
        decode_to_slice(v, &mut bytes).map_err(|e| E::custom(format!("Hex decode error: {e}")))?;
        Ok(Bytes32(bytes))
    }
}

impl<'de> Visitor<'de> for Bytes64HexVisitor {
    type Value = Bytes64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 128-character hex string for a 64 byte array")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut bytes = [0u8; 64];
        decode_to_slice(v, &mut bytes).map_err(|e| E::custom(format!("Hex decode error: {e}")))?;
        Ok(Bytes64(bytes))
    }
}

impl<'de> Deserialize<'de> for Bytes32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Bytes32HexVisitor)
    }
}

impl<'de> Deserialize<'de> for Bytes64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Bytes64HexVisitor)
    }
}

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
};
use std::{convert::{AsRef, TryFrom}, error::Error};

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

impl PgHasArrayType for Bytes32 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_bytes32")
    }
}

impl Type<Postgres> for Bytes32 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("bytes32")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || *ty == PgTypeInfo::with_name("bytea")
    }
}

impl Encode<'_, Postgres> for Bytes32 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
        <&[u8] as Encode<Postgres>>::encode(self.0.as_slice(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for Bytes32 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let raw: &[u8] = Decode::<'r, Postgres>::decode(value)?;
        raw.try_into()
            .map_err(|_| format!("Expected exactly 32 bytes, got {}", raw.len()).into())
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

impl Type<Postgres> for Bytes64 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("bytes64")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || *ty == PgTypeInfo::with_name("bytea")
    }
}

impl Encode<'_, Postgres> for Bytes64 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Box<dyn Error + Send + Sync + 'static>> {
        <&[u8] as Encode<Postgres>>::encode(self.0.as_slice(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for Bytes64 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let raw: &[u8] = Decode::<'r, Postgres>::decode(value)?;
        raw.try_into()
            .map_err(|_| format!("Expected exactly 64 bytes, got {}", raw.len()).into())
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
