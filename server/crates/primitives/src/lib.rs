pub mod hex_bytes;
pub mod impls;
mod repr_bytes;

pub use repr_bytes::ReprBytes;

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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
