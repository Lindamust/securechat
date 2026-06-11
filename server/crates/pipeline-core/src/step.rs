use crate::{
    error::PipelineResult,
    hlist::{Extracts, Prepends},
};
use frunk::hlist::HList;

// marker trait for async runtimes
pub trait ExecutorFor<S: ?Sized> {}

// force everything to be async (non async returns core::future::ready)
pub trait Step {
    type Needs: HList + Send;
    type Provides: Send;

    fn run_step<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Ctx::Remainder as Prepends<Self::Provides>>::Output>> + Send
    where
        Ctx: HList + Extracts<Self::Needs, Idx> + Send,
        Ctx::Remainder: HList + Prepends<Self::Provides> + Send,
        <Ctx::Remainder as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>;
}
