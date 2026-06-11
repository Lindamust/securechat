use chrono::{Duration, Utc};
use infra::crypto::verify_signed_nonce;
use infra::database::{NonceRow, PgDatabase};
use pipeline_core::error::PipelineError;
use pipeline_core::hlist::Prepends;
use pipeline_core::hlist_pat;
use pipeline_core::{
    HList, HNil,
    error::PipelineResult,
    hlist::{SculptedRemainder, Sculptor},
    step::{ExecutorFor, Step},
};

use domain::dto::SignedTokenBody;

use pipeline_http::extractors::auth::mint_jwt;

use uuid::Uuid;

use domain::dto::AuthChallengeBody;
use domain::models::{NonceKey, NonceType};

// -------- auth/challenge --------
// visibility: public

/// pure: makes random nonce
/// needs: none,
/// provides: NonceType
#[derive(Clone)]
pub struct GenerateNonce;

impl Step for GenerateNonce {
    type Needs = HNil;
    type Provides = NonceType;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        _: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>,
    {
        async move {
            let (_, rem) = ctx.sculpt();
            let nonce = NonceType {
                nonce: NonceKey::generate(),
                expires_at: Utc::now() + Duration::seconds(30),
            };
            Ok(rem.prepend_type(nonce))
        }
    }
}

// async: inserts nonce
// needs: NonceType, auth req body
// provides: NonceKey
#[derive(Clone)]
pub struct StoreNonce;

impl Step for StoreNonce {
    type Needs = HList![NonceType, AuthChallengeBody]; // non-hnil needs
    type Provides = NonceKey;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>,
    {
        async move {
            let (hlist_pat![nonce_type, auth_req], rem) = ctx.sculpt();

            // SAFETY: PgDatabase implements ExecutorFor<StoreNonce>
            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase) };

            let ik_pub = &auth_req.ik_pub;

            let nonce_key = pg.store_nonce(ik_pub, &nonce_type).await?;

            Ok(rem.prepend_type(nonce_key))
        }
    }
}

// -------- auth/token --------
// visibility: public

/// async: gets stored nonce
/// needs: ReqBody (for the IkPub inside)
/// provides: VerifyBody (ReqBody + db row with user uuid)
#[derive(Clone)]
pub struct GetNonce;

pub struct VerifyBody {
    nonce_row: NonceRow,
    req_body: SignedTokenBody,
}

impl Step for GetNonce {
    type Needs = HList![SignedTokenBody];
    type Provides = VerifyBody;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>,
    {
        async move {
            let (hlist_pat![req_body], rem) = ctx.sculpt();
            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase) };
            let nonce_row = pg.get_nonce(&req_body.ik_pub).await?;
            let verify_body = VerifyBody {
                nonce_row,
                req_body,
            };

            Ok(rem.prepend_type(verify_body))
        }
    }
}

/// pure: verify sigdata
/// needs: VerifyBody
/// provides: VerifyBody
#[derive(Clone)]
pub struct VerifySignedNonce;

impl Step for VerifySignedNonce {
    type Needs = HList![VerifyBody];
    type Provides = Uuid;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        _: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>,
    {
        async move {
            let (hlist_pat![verify_body], rem) = ctx.sculpt();

            let ik_pub = &verify_body.req_body.ik_pub;
            let nonce = &verify_body.nonce_row.nonce;
            let sig = &verify_body.req_body.sig_data;

            match verify_signed_nonce(ik_pub, nonce, sig) {
                true => {}
                false => return Err(pipeline_core::error::PipelineError::Forbidden),
            }

            let uuid = verify_body.nonce_row.user_id;

            Ok(rem.prepend_type(uuid))
        }
    }
}

/// pure: make new jwt
/// needs: uuid,
/// provides: JwtToken,
#[derive(Clone)]
pub struct MintJwt;

impl Step for MintJwt {
    type Needs = HList![Uuid];
    type Provides = String;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        _: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>,
    {
        async move {
            let (hlist_pat![uuid], rem) = ctx.sculpt();

            let jwt = mint_jwt(uuid).map_err(|_| PipelineError::Forbidden)?;

            Ok(rem.prepend_type(jwt))
        }
    }
}
