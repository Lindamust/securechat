use frunk::{
    HCons,
    hlist::{HList, Sculptor}, 
};

use std::future::Future;

use crate::error::PipelineResult;

pub trait ExecutorFor<S: ?Sized> {}

pub trait PureStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx>(self, ctx: H) -> PipelineResult<HCons<Self::Provides, <H as Sculptor<Self::Needs, Idx>>::Remainder>>
    where
        H: Sculptor<Self::Needs, Idx>;
}

pub trait AsyncStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<Self::Provides, <H as Sculptor<Self::Needs, Idx>>::Remainder>>> + Send
    where
        H: Sculptor<Self::Needs, Idx> + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}


impl<S: PureStep + Send> AsyncStep for S
{
    type Needs = S::Needs;
    type Provides = S::Provides;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<Self::Provides, <H as Sculptor<Self::Needs, Idx>>::Remainder>>> + Send
    where
        H: Sculptor<S::Needs, Idx> + Send,
        Exec: ExecutorFor<S> + ?Sized + Sync,
    {
        async move { PureStep::run(self, ctx) }
    }
}

