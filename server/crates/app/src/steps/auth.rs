use chrono::{Duration, Utc};
use pipeline_core::{HCons, HList, HNil, hlist_macro, step::PureStep};

use domain::models::{NonceKey, NonceType};

#[derive(Clone)]
pub struct GenerateNonce;

impl PureStep for GenerateNonce {
    type Needs = HList![HNil];
    type Provides = HList![NonceType];

    type Remainder<H, Idx> = H
        where
            H: hlist_macro::Sculptor<Self::Needs, Idx> + HList;

    fn run<H, Idx>(
        self,
        ctx: H,
    ) -> pipeline_core::error::PipelineResult<pipeline_core::HCons<Self::Provides, H>>
    where
        H: pipeline_core::Sculptor<Self::Needs, Idx>,
    {
        let nonce = NonceType {
            nonce: NonceKey::generate(),
            expires_at: Utc::now() + Duration::seconds(30),
        };

        Ok(HCons {
            head: hlist_macro![nonce],
            tail: ctx,
        })
    }
}
