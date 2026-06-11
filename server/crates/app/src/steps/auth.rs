use chrono::{Duration, Utc};
use pipeline_core::{HCons, HList, HNil, hlist_macro, step::PureStep, error::PipelineResult};

use domain::models::{NonceKey, NonceType};

// -------- auth/challenge --------
// visibility: public

/// pure: makes random nonce
/// needs: none,
/// provides: NonceType
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

/// async: inserts nonce
/// needs: NonceType
/// provides: NonceKey
#[derive(Clone)]
pub struct StoreNonce;


// -------- auth/token --------
// visibility: public

/// async: gets stored nonce
/// needs: SigBody (for the IkPub inside)
/// provides: VerifyBody (SigBody + db row with user uuid)
#[derive(Clone)]
pub struct GetNonce;

/// pure: verify sigdata
/// needs: VerifyBody
/// provides: VerifyBody
#[derive(Clone)]
pub struct VerifySignedNonce;

/// pure: make new jwt
/// needs: VerifyBody,
/// provides: JwtToken,
#[derive(Clone)]
pub struct MintJwt;
