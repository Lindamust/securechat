use super::{Bytes32, Bytes64};

pub trait ReprBytes<const N: usize> {
    fn bytes_const(&self) -> &[u8; N];
    fn bytes_as_slice(&self) -> &[u8];
}

macro_rules! impl_repr_bytes {
    ($( $t:ident < $n:literal > ),* $(,)?) => {
        $(
            impl ReprBytes<$n> for super::$t {
                fn bytes_const(&self) -> &[u8; $n] {
                    &self.0.0
                }

                fn bytes_as_slice(&self) -> &[u8] {
                    self.0.as_ref()
                }
            }
        )*
    };
}

macro_rules! impl_from_vec {
    ($( ($t:ident, $inner:ident) ),* $(,)?) => {
        $(
            impl From<Vec<u8>> for super::$t {
                fn from(v: Vec<u8>) -> Self {
                    let bytes = super::$inner::try_from(v).unwrap_or_else(|_| {
                        panic!("{}: invalid byte length from database", stringify!($t))
                    });

                    Self(bytes)
                }
            }
        )*
    };
}

impl_repr_bytes!(
    IkPub<32>,
    IkPubEd<32>,
    SpkPub<32>,
    SpkPubSig<64>,
    SigData<64>,
    OtpkPub<32>,
    Nonce<32>,
);

impl_from_vec!(
    (IkPub, Bytes32),
    (IkPubEd, Bytes32),
    (SpkPub, Bytes32),
    (SpkPubSig, Bytes64),
    (SigData, Bytes64),
    (OtpkPub, Bytes32),
    (Nonce, Bytes32),
);

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

impl TryFrom<&[u8]> for Bytes64 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl TryFrom<Vec<u8>> for Bytes64 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let arr: [u8; 64] = value.as_slice().try_into()?;
        Ok(Self(arr))
    }
}

impl AsRef<[u8]> for Bytes64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
