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

    fn run<H, Idx>(self, ctx: H) -> PipelineResult<HCons<Self::Provides, H::Remainder>>
    where
        H: Sculptor<Self::Needs, Idx>,
        <H as Sculptor<Self::Needs, Idx>>::Remainder: Send;
}

pub trait AsyncStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<Self::Provides, H::Remainder>>> + Send
    where
        H: Sculptor<Self::Needs, Idx> + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync,
        <H as Sculptor<Self::Needs, Idx>>::Remainder: Send;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

impl<S> AsyncStep for S
where
    S: PureStep + Send,
{
    type Needs = S::Needs;
    type Provides = S::Provides;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<S::Provides, H::Remainder>>> + Send
    where
        H: Sculptor<S::Needs, Idx> + Send,
        Exec: ExecutorFor<S> + ?Sized + Sync,
        <H as Sculptor<S::Needs, Idx>>::Remainder: Send,
    {
        async move { PureStep::run::<H, Idx>(self, ctx) }
    }
}

trait StepTypes {
    type Needs: HList;
    type Provides: HList;
}
