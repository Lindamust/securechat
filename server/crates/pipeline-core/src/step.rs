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

    type Remainder<H, Idx>: HList
    where
        H: Sculptor<Self::Needs, Idx>;

    fn run<H, Idx>(self, ctx: H) -> PipelineResult<HCons<Self::Provides, Self::Remainder<H, Idx>>>
    where
        H: Sculptor<Self::Needs, Idx>;
}

pub trait PureConsumingStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx, Rem>(self, ctx: H) -> PipelineResult<HCons<Self::Provides, Rem>>
    where
        H: Sculptor<Self::Needs, Idx, Remainder = Rem>;
}

pub trait AsyncStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<Self::Provides, H>>> + Send
    where
        H: Sculptor<Self::Needs, Idx> + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync;
}

pub trait AsyncConsumingStep {
    type Needs: HList;
    type Provides: HList;

    fn run<H, Idx, Exec, Rem>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<Self::Provides, Rem>>> + Send
    where
        H: Sculptor<Self::Needs, Idx, Remainder = Rem> + Send,
        Exec: ExecutorFor<Self> + ?Sized + Sync;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

impl<S: PureStep + Send> AsyncStep for S {
    type Needs = <S as PureStep>::Needs;
    type Provides = <S as PureStep>::Provides;

    fn run<H, Idx, Exec>(
        self,
        ctx: H,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<HCons<<S as PureStep>::Provides, H>>> + Send
    where
        H: Sculptor<<S as PureStep>::Needs, Idx> + Send,
        Exec: ExecutorFor<S> + ?Sized + Sync,
    {
        async move { PureStep::run(self, ctx) }
    }
}

// impl<Exec, S: PureConsumingStep> ExecutorFor<S> for Exec
