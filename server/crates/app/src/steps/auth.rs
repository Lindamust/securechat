use chrono::{Duration, Utc};
use pipeline_core::{HCons, HList, HNil, hlist_macro, step::PureStep, error::PipelineResult};

use domain::models::{NonceKey, NonceType};
use domain::dto::{AuthChallengeBody};

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
/// needs: NonceType, auth req body
/// provides: NonceKey
#[derive(Clone)]
pub struct StoreNonce;

impl AsyncStep for StoreNonce {
    type Needs: HList![NonceType, AuthChallengeBody];
    type Provides: NonceKey;

    fn run_async<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + Sculptor<Self::Target, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync,
    {
        async move {
            let (hlist_pat![nonce_type, auth_body], rem) = ctx.sculpt();

            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase)  };
            let nonce_key = pg.insert_nonce(&auth_body.ik_pub, &nonce_type).await?;

            Ok(rem.prepend_type(nonce_key))
        }
    }
}


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
