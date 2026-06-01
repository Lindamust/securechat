use infra::database::{InsertsUser, PgDatabase};
use pipeline_core::{
    HCons, HList, hlist_macro, hlist_pat,
    step::{AsyncStep, ExecutorFor, PureStep},
};

use domain::dto::{InsertedUser, NewUser, RegisterBody};

#[derive(Clone)]
pub struct HashPassword;

impl PureStep for HashPassword {
    type Needs = HList![RegisterBody];
    type Provides = HList![NewUser];

    fn run<H, Idx>(
        self,
        ctx: H,
    ) -> pipeline_core::error::PipelineResult<pipeline_core::HCons<Self::Provides, H::Remainder>>
    where
        H: pipeline_core::Sculptor<Self::Needs, Idx>,
    {
        let (hlist_pat![reg_body], remainder) = ctx.sculpt();

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

        Ok(HCons {
            head: hlist_macro![new_user],
            tail: remainder,
        })
    }
}

#[derive(Clone)]
pub struct StoreUser;

impl AsyncStep for StoreUser {
    type Needs = HList![NewUser];
    type Provides = HList![InsertedUser];

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<
        Output = pipeline_core::error::PipelineResult<HCons<Self::Provides, H::Remainder>>,
    > + Send
    where
        H: hlist_macro::Sculptor<Self::Needs, Idx> + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync,
    {
        async move {
            let (hlist_pat![new_user], remainder) = ctx.sculpt();

            let pg = unsafe { &*(executor as *const Exec as *const PgDatabase) };
            let res = pg.insert_user(&new_user).await?;

            Ok(HCons {
                head: hlist_macro![res],
                tail: remainder,
            })
        }
    }
}

// use infra::database::InsertsUser;

// impl<T: InsertsUser> ExecutorFor<StoreUser> for T {}
