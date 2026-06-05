use frunk::{
    hlist::{HList, Sculptor}
};

use std::future::Future;

use crate::error::PipelineResult;

pub trait ExecutorFor<S: ?Sized> {}

pub trait PureStep {
    type Needs;
    type Provides;

    type Output<Ctx>: HList
    where
        Ctx: HList;

    fn run<Ctx, Idx>(self, ctx: Ctx) -> PipelineResult<Self::Output<Ctx>>
    where
        Ctx: Sculptor<Self::Needs, Idx> + HList;
}

pub trait AsyncStep {
    type Needs;
    type Provides;

    type Output<Ctx>: HList
    where
        Ctx: HList;

    fn run<Ctx, Idx, Exec>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output<Ctx>>> + Send
    where
        Ctx: Sculptor<Self::Needs, Idx> + HList + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

impl<S: PureStep + Send> AsyncStep for S
{
    type Needs = S::Needs;
    type Provides = S::Provides;

    type Output<Ctx> = S::Output<Ctx>
    where
        Ctx: HList;

    fn run<Ctx, Idx, Exec>(
        self,
        ctx: Ctx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output<Ctx>>> + Send
    where
        Ctx: Sculptor<Self::Needs, Idx> + HList + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync
    {
        async move {
            PureStep::run(self, ctx)
        }
    }
}
