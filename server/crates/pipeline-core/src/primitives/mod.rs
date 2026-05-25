mod bytes_impl;
mod from_str;
pub(self) mod hex_bytes;
mod primitives;

pub use bytes_impl::ReprBytes;
pub use primitives::*;

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
