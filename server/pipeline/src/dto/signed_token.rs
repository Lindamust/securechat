use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extractors::auth::Claims,
    primitives::{IkPub, SigData},
};

#[derive(Debug, Deserialize, Validate)]
pub struct SignedTokenBody {
    pub ik_pub: IkPub,
    pub sig_data: SigData,
}

#[derive(Debug, Serialize)]
pub struct SignedTokenResponse {
    bearer: Claims,
}
