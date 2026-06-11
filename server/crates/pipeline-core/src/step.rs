use frunk::hlist::{HList, Sculptor};
use crate::{error::PipelineResult, hlist::Prepends};

// marker trait for async runtimes
pub trait ExecutorFor<S: ?Sized> {}

pub trait PureStep {
    type Needs: HList;  // can pull ouy any number of types
    type Provides;      // but can add only one

    fn run_pure<Ctx, Idx>(
        self,
        ctx: Ctx,
    ) -> PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>
    where
        Ctx: HList + Sculptor<Self::Needs, Idx>,
        Ctx::Remainder: HList + Prepends<Self::Provides>,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList;
}

// every pure step is executable by any runtime
impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

// same as PureStep except it returns a future
pub trait AsyncStep {
    type Needs: HList;
    type Provides;

    fn run_async<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>;
}

// coerce every pure step into an AsyncStep with an immeadately available future
impl<S: PureStep + Send> AsyncStep for S {
    type Needs = <S as PureStep>::Needs;
    type Provides = <S as PureStep>::Provides;

    fn run_async<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<<S as PureStep>::Provides>>::Output>> + Send
    where
        Ctx: HList + Sculptor<<S as PureStep>::Needs, Idx> + Send,
        Ctx::Remainder: HList + Prepends<<S as PureStep>::Provides> + Send,
        <Ctx::Remainder as Prepends<<S as PureStep>::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self> 
    {
        core::future::ready(self.run_pure(ctx))
    }
}

