pub trait Stage: private::Sealed {}

/// Raw bytes/JSON parsed by Axum extractors
pub struct Raw;

/// A valid JWT is present or the endpoint is Pub
pub struct Authenticated;

/// Domain invariants enforced -> domain types
pub struct Validated;

/// Raw infra result
pub struct Executed;

/// Result mapped to HTTP response body DTO
pub struct Responded;

macro_rules! blanket_impl {
    ($trait:ident => $($ty:ident),* $(,)?) => {
        $(impl $trait for $ty {})*
    };
}

blanket_impl!(Stage =>
    Raw,
    Authenticated,
    Validated,
    Executed,
    Responded,
);

mod private {
    use super::{Authenticated, Executed, Raw, Responded, Validated};

    pub trait Sealed {}
    blanket_impl!(Sealed =>
        Raw,
        Authenticated,
        Validated,
        Executed,
        Responded,
    );
}
