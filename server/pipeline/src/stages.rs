pub trait Stage: private::Sealed {}

/// Raw bytes/JSON parsed by Axum extractors
pub struct Raw;

/// A valid JWT is present or the endpoint is Pub
pub struct Authenticated;

/// JSON body deserialised
pub struct Dto;

/// Domain invariants enforced -> domain types
pub struct Validated;

/// Strongly-typed `Command` for validated data; return type encoded
pub struct CommandReady;

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
    Dto,
    Validated,
    CommandReady,
    Executed,
    Responded,
);

mod private {
    use super::{Authenticated, CommandReady, Dto, Executed, Raw, Responded, Validated};

    pub trait Sealed {}
    blanket_impl!(Sealed =>
        Raw,
        Authenticated,
        Dto,
        Validated,
        CommandReady,
        Executed,
        Responded,
    );
}
