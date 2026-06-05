use frunk::{
    hlist::{HList, Sculptor}
};

use std::future::Future;

use crate::error::PipelineResult;

pub trait ExecutorFor<S: ?Sized> {}

pub trait PureStep {
    type Needs;
    type Provides;

    fn run<OldCtx, NewCtx, Idx>(self, ctx: OldCtx) -> PipelineResult<NewCtx>
    where
        OldCtx: Sculptor<Self::Needs, Idx>,
        NewCtx: HList;
}

pub trait AsyncStep {
    type Needs;
    type Provides;

    fn run<OldCtx, NewCtx, Idx, Exec>(
        self,
        ctx: OldCtx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<NewCtx>> + Send
    where
        OldCtx: Sculptor<Self::Needs, Idx> + Send,
        NewCtx: HList + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

impl<S: PureStep + Send> AsyncStep for S
{
    type Needs = S::Needs;
    type Provides = S::Provides;

    fn run<OldCtx, NewCtx, Idx, Exec>(
        self,
        ctx: OldCtx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<NewCtx>> + Send
    where
        OldCtx: Sculptor<Self::Needs, Idx> + Send,
        NewCtx: HList + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync
    {
        async move {
            PureStep::run(self, ctx)
        }
    }
}
