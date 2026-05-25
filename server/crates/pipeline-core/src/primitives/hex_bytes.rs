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
