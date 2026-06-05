use chrono::{Duration, Utc};
use pipeline_core::{HCons, HList, HNil, hlist_macro, step::PureStep, error::PipelineResult};

use domain::models::{NonceKey, NonceType};

#[derive(Clone)]
pub struct GenerateNonce;

impl PureStep for GenerateNonce {
    type Needs = HNil;
    type Provides = NonceType;

    type Output<Ctx> = HCons<NonceType, Ctx> where Ctx: HList;

    fn run<Ctx, Idx>(self, ctx: Ctx) -> PipelineResult<Self::Output<Ctx>>
        where
            Ctx: hlist_macro::Sculptor<Self::Needs, Idx> + HList
    {
        let nonce = NonceType {
            nonce: NonceKey::generate(),
            expires_at: Utc::now() + Duration::seconds(30),
        };

        Ok(ctx.prepend(nonce))
    }
}
