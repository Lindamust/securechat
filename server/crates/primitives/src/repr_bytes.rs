pub trait ReprBytes<const N: usize> {
    fn bytes_const(&self) -> &[u8; N];
    fn bytes_as_slice(&self) -> &[u8];
}

#[macro_export]
macro_rules! impl_repr_bytes {
    ($( $t:ident < $n:literal > ),* $(,)?) => {
        $(
            impl ReprBytes<$n> for $t {
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

#[macro_export]
macro_rules! impl_from_vec {
    ($( ($t:ident, $inner:ident) ),* $(,)?) => {
        $(
            impl From<Vec<u8>> for $t {
                fn from(v: Vec<u8>) -> Self {
                    let bytes = $inner::try_from(v).unwrap_or_else(|_| {
                        panic!("{}: invalid byte length from database", stringify!($t))
                    });

                    Self(bytes)
                }
            }
        )*
    };
}
