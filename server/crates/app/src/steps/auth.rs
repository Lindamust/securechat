use chrono::{Duration, Utc};
// use infra::crypto::verify_signed_nonce;
// use infra::database::NonceRow;
use pipeline_core::hlist::Prepends;
// use pipeline_core::hlist_pat;
use pipeline_core::{HList, HNil, error::PipelineResult, step::Step};

// use pipeline_http::extractors::auth::mint_jwt;

// use uuid::Uuid;

// use domain::dto::{AuthChallengeBody, SignedTokenBody};
use domain::models::{NonceKey, NonceType};
// use pipeline_http::extractors::auth::Claims;

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

    fn run_step<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        _: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + pipeline_core::hlist::Extracts<Self::Needs, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + pipeline_core::step::ExecutorFor<Self>,
    {
        async move {
            let (_, rem) = ctx.extract_types();

            let nonce = NonceType {
                nonce: NonceKey::generate(),
                expires_at: Utc::now() + Duration::seconds(30),
            };

            Ok(rem.prepend_type(nonce))
        }
    }
}

/// async: inserts nonce
/// needs: NonceType, auth req body
/// provides: NonceKey
#[derive(Clone)]
pub struct StoreNonce;

// impl AsyncStep for StoreNonce {
//     type Needs = HList![NonceType, AuthChallengeBody];
//     type Provides = NonceKey;

//     fn run_async<Ctx, Exec, Idx>(
//         self,
//         ctx: Ctx,
//         executor: &Exec,
//     ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
//     where
//         Ctx: HList + Sculptor<Self::Target, Idx> + Send,
//         Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
//         <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
//         Exec: ExecutorFor<Self> + ?Sized + Sync,
//     {
//         async move {
//             let (hlist_pat![nonce_type, auth_body], rem) = ctx.sculpt();

//             let pg = unsafe { &*(executor as *const Exec as *const PgDatabase) };
//             let nonce_key = pg.insert_nonce(&auth_body.ik_pub, &nonce_type).await?;

//             Ok(rem.prepend_type(nonce_key))
//         }
//     }
// }

// -------- auth/token --------
// visibility: public

// async: gets stored nonce
// needs: ReqBody (for the IkPub inside)
// provides: VerifyBody (ReqBody + db row with user uuid)
// #[derive(Clone)]
// pub struct GetNonce;

// pub struct VerifyBody {
//     nonce_row: NonceRow,
//     req_body: SignedTokenBody,
// }

// impl AsyncStep for GetNonce {
//     type Needs = HList![SignedTokenBody];
//     type Provides = VerifyBody;

//     fn run_async<Ctx, Exec, Idx>(
//         self,
//         ctx: Ctx,
//         executor: &Exec,
//     ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
//     where
//         Ctx: HList + Sculptor<Self::Target, Idx> + Send,
//         Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
//         <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
//         Exec: ExecutorFor<Self> + ?Sized + Sync,
//     {
//         async move {
//             let (hlist_pat![req_body], rem) = ctx.sculpt();
//             let pg = unsafe { &*(executor as *const Exec as *const PgDatabase) };
//             let nonce_row = pg.get_nonce(&req_body.ik_pub).await?;
//             let verify_body = VerifyBody {
//                 nonce_row,
//                 req_body,
//             };

//             Ok(rem.prepend_type(verify_body))
//         }
//     }
// }

// pure: verify sigdata
// needs: VerifyBody
// provides: VerifyBody
// #[derive(Clone)]
// pub struct VerifySignedNonce;

// impl PureStep for VerifySignedNonce {
//     type Needs = HList![VerifyBody];
//     type Provides = Uuid;

//     fn run_pure<Ctx, Idx>(
//         self,
//         ctx: Ctx,
//     ) -> PipelineResult<<Ctx::Remainder as pipeline_core::hlist::Prepends<Self::Provides>>::Output>
//     where
//         Ctx: HList + hlist_macro::Sculptor<Self::Needs, Idx>,
//         Ctx::Remainder: HList + pipeline_core::hlist::Prepends<Self::Provides>,
//         <Ctx::Remainder as pipeline_core::hlist::Prepends<Self::Provides>>::Output: HList,
//     {
//         let (hlist_pat![verify_body], rem) = ctx.sculpt();

//         let ik_pub = &verify_body.req_body.ik_pub;
//         let nonce = &verify_body.nonce_row.nonce_key;
//         let sig = &verify_body.req_body.sig_data;

//         match verify_signed_nonce(ik, nonce, sig) {
//             true => {}
//             false => return Err(pipeline_core::error::PipelineError::Forbidden),
//         }

//         let uuid = verify_body.nonce_row.user_uuid;

//         Ok(rem.prepend_type(uuid))
//     }
// }

// pure: make new jwt
// needs: uuid,
// provides: JwtToken,
// #[derive(Clone)]
// pub struct MintJwt;

// impl PureStep for MintJwt {
//     type Needs = HList![Uuid];
//     type Provides = String;

//     fn run_pure<Ctx, Idx>(
//         self,
//         ctx: Ctx,
//     ) -> PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>
//     where
//         Ctx: HList + hlist_macro::Sculptor<Self::Needs, Idx>,
//         Ctx::Remainder: HList + Prepends<Self::Provides>,
//         <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList,
//     {
//         let (hlist_pat![uuid], rem) = ctx.sculpt();

//         let jwt = mint_jwt(uuid)?;

//         Ok(rem.prepend_type(jwt))
//     }
// }
