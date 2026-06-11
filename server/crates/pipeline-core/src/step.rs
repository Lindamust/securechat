use core::ops::Add;

use frunk::hlist::{HList, Sculptor};

use crate::{error::PipelineResult, hlist::Prepends};

pub trait ExecutorFor<S: ?Sized> {}

pub trait PureStep {
    type Needs: HList;
    type Provides;

    fn run_pure<Ctx, Idx>(
        self,
        ctx: Ctx,
    ) -> PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>
    where
        Ctx: HList + Sculptor<Self::Target, Idx>,
        Ctx::Remainder: HList + Prepends<Self::Provides>,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

pub trait AsyncStep {
    type Needs: HList;
    type Provides;

    fn run_pure<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + Sculptor<Self::Target, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>;
}

impl<S: PureStep + Send> AsyncStep for S {
    type Needs = <S as PureStep>::Needs;
    type Provides = <S as PureStep>::Provides;

    fn run_pure<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + Sculptor<Self::Target, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self> 
    {
        core::future::ready(PureStep::run(self, ctx))
    }
}

