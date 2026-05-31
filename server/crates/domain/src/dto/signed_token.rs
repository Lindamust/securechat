use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{IkPub, SigData};

// http request + response

#[derive(Debug, Deserialize, Validate)]
pub struct SignedTokenBody {
    pub ik_pub: IkPub,
    pub sig_data: SigData,
}

#[derive(Debug, Serialize)]
pub struct SignedTokenResponse {
    pub token: String,
}

// // infra layer return type

// #[derive(Debug)]
// pub struct VerifySignResult {
//     pub is_valid: bool,
// }

// // Request<Validated, I> ---> Command ---> Request<Executed, O>
// // Step 1: Get the Nonce
// // Step 2: verify it
// impl InfraCommand for SignedTokenBody {
//     type Output = VerifySignResult;
// }

// impl IntoCommand for SignedTokenBody {
//     type Command = Self;
//     fn into_command(self, _idenity: &crate::extractors::auth::Identity) -> Self::Command {
//         self
//     }
// }
