use crate::{
    error::PipelineResult,
    hlist::{Prepends, SculptedRemainder, Sculptor},
};
use frunk::hlist::HList;

// marker trait for async runtimes
pub trait ExecutorFor<S: ?Sized> {}

// force everything to be async (non async returns core::future::ready)
pub trait Step {
    type Needs: HList + Send;
    type Provides: Send;

    fn run_step<Ctx, Rem, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<
        Output = PipelineResult<<SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output>,
    > + Send
    where
        Ctx: HList + Sculptor<Self::Needs, Idx, Remainder = SculptedRemainder<Rem>> + Send,
        Rem: HList + Send,
        SculptedRemainder<Rem>: HList + Prepends<Self::Provides> + Send,
        <SculptedRemainder<Rem> as Prepends<Self::Provides>>::Output: HList + Send,
        Exec: ?Sized + Sync + ExecutorFor<Self>;
}
