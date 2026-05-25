use crate::{
    primitives::{IkPub, Nonce},
    traits::InfraCommand,
    traits::IntoCommand,
};

use serde::{Deserialize, Serialize};
use validator::Validate;

// HTTP request + response body

#[derive(Debug, Deserialize, Validate)]
pub struct AuthChallengeBody {
    pub ik_pub: IkPub,
}

#[derive(Debug, Serialize)]
pub struct AuthChallengeResponse {
    pub nonce: Nonce,
}

// Infra layer return type

#[derive(Debug)]
pub struct AuthChallengeNonce {
    pub nonce: Nonce,
}

// Request<Validated, I> ---> Command ---> Request<Executed, O>

impl InfraCommand for AuthChallengeBody {
    type Output = AuthChallengeNonce;
}

impl IntoCommand for AuthChallengeBody {
    type Command = Self;
    fn into_command(self, _idenity: &crate::extractors::auth::Identity) -> Self::Command {
        self
    }
}
