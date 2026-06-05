use infra::database::{InsertsUser, PgDatabase};
use pipeline_core::{
    HCons, HList, Sculptor, hlist_macro::{self, Plucker}, hlist_pat, step::{AsyncStep, PureStep},
};

use domain::dto::{NewUser, RegisterBody, InsertedUser};

#[derive(Clone)]
pub struct HashPassword;

impl PureStep for HashPassword {
    type Needs = HList![RegisterBody];
    type Provides = NewUser;

    type Output<Ctx> = HCons<NewUser, <Ctx as Plucker<RegisterBody, _>>::Remainder>
        where
            Ctx: HList;

    fn run<Ctx, Idx>(self, ctx: Ctx) -> pipeline_core::error::PipelineResult<Self::Output<Ctx>>
        where
            Ctx: Sculptor<Self::Needs, Idx> + HList,
    {
        let (hlist_pat![a], r) = ctx.sculpt();

        let hashed_password = a.password.hash();

        let new_user = NewUser {
            username: a.username,
            email: a.email,
            hashed_password,
            ik_pub: a.ik_pub,
            ik_pub_ed: a.ik_pub_ed,
            spk_pub: a.spk_pub,
            spk_pub_sig: a.spk_pub_sig,
            otpks: a.otpks,
        };

        Ok(hlist_macro![new_user, r])
    }
}

#[derive(Clone)]
pub struct StoreUser;

impl AsyncStep for StoreUser {
    type Needs = HList![NewUser];
    type Provides = HList![InsertedUser];

    type Output<Ctx> = HCons<InsertedUser, Ctx>
        where
            Ctx: HList;

    fn run<Ctx, Idx, Exec>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = pipeline_core::error::PipelineResult<Self::Output<Ctx>>> + Send
    where
        Ctx: Sculptor<Self::Needs, Idx> + HList + Send,
        Exec: pipeline_core::step::ExecutorFor<Self> + ?Sized + Sync
    {
        async move {
            let (hlist_pat![new_user], remainder) = ctx.sculpt();

            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase)  };
            let res = pg.insert_user(&new_user).await?;

            Ok(HCons { head: hlist_macro![res], tail: remainder })
        }
    }
}

