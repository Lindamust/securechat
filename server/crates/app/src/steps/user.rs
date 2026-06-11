use infra::database::{InsertsUser, PgDatabase};
use pipeline_core::{
    HCons, HList, Sculptor, hlist_macro::{self, Plucker}, hlist_pat, step::{AsyncStep, PureStep},
};

use domain::dto::{NewUser, RegisterBody, InsertedUser};

// -------- api/register --------
// visibility: public

#[derive(Clone)]
pub struct HashPassword;

impl PureStep for HashPassword {
    type Needs = HList![RegisterBody];
    type Provides = NewUser;

    fn run_pure<Ctx, Idx>(
        self,
        ctx: Ctx,
    ) -> PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>
    where
        Ctx: HList + Sculptor<Self::Target, Idx>,
        Ctx::Remainder: HList + Prepends<Self::Provides>,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList
    {
        let (hlist_pat![reg_body], rem) = ctx.sculpt();
        let hashed_password = reg_body.password.hash();

        let new_user = NewUser {
            username: reg_body.username,
            email: reg_body.email,
            hashed_password,
            ik_pub: reg_body.ik_pub,
            ik_pub_ed: reg_body.ik_pub_ed,
            spk_pub: reg_body.spk_pub,
            spk_pub_sig: reg_body.spk_pub_sig,
            otpks: reg_body.otpks,
        };

        Ok(rem.prepend_type(new_user))
    }
}

#[derive(Clone)]
pub struct StoreUser;

impl AsyncStep for StoreUser {
    type Needs = HList![NewUser];
    type Provides = InsertedUser;

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
            let (hlist_pat![new_user], rem) = ctx.sculpt();

            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase)  };
            let res = pg.insert_user(&new_user).await?;

            Ok(rem.prepend_type(res))
        }
    }
}

// -------- api/fetch_prekeys --------
// visibility: private

// async: fetch from db
// needs: target username
// provides: prekey batch
#[derive(Clone)]
pub struct GetPrekeys;


// -------- api/replenish_otpks --------
// visibility: private

// async: insert into db
// needs: target uuid, otpks vec
// provides: acknowledgement (insert count)
#[derive(Clone)]
pub struct AddOtpks;

// -------- api/otpk_count --------
// visibility: private

// async: fetch from db
// needs: target uuid
// provides: count (i64)
#[derive(Clone)]
pub struct CheckOtpkCount;

// -------- api/rotate_spk --------
// visibility: private


// async: insert into db
// needs: target uuid, spk pub, spk pub sig
// provides: acknowledgement (timestamp)
#[derive(Clone)]
pub struct ChangeSpk;