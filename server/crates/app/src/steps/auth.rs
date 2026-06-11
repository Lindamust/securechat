use chrono::{Duration, Utc};
use pipeline_core::{HCons, HList, HNil, hlist_macro, step::PureStep, error::PipelineResult};

use domain::models::{NonceKey, NonceType};

#[derive(Clone)]
pub struct GenerateNonce;

impl PureStep for GenerateNonce {
    type Needs = HNil;
    type Provides = NonceType

    fn run_pure<Ctx, Idx>(
        self,
        ctx: Ctx,
    ) -> PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>
    where
        Ctx: HList + Sculptor<Self::Target, Idx>,
        Ctx::Remainder: HList + Prepends<Self::Provides>,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList
    {
        let (_, rem) = ctx.sculpt();
        let nonce_type = NonceType {
            nonce: NonceKey::generate(),
            expires_at: Utc::now() + Duration::seconds(30),
        };

        Ok(rem.prepend_type(nonce_type))
    }
}

#[derive(Clone)]
pub struct VerifySignedNonce;
