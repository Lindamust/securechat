use core::future::Future;
use core::marker::PhantomData;

use frunk::{
    HNil,
    hlist::{HList, Sculptor},
};

use crate::{
    error::PipelineResult,
    hlist::Prepends,
    step::{AsyncStep, ExecutorFor},
};

/// Then composition node
#[derive(Clone)]
pub struct Then<A, B, Idx>(pub A, pub B, PhantomData<Idx>);

//// recursive execution trait
pub trait ExecuteChain<Ctx, Exec: ?Sized + Sync> {
    type ChainOutput;

    fn execute(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::ChainOutput>> + Send;
}

/// Base case: empty chain, returns context unchanged.
impl<Ctx: Send, Exec: ?Sized + Sync> ExecuteChain<Ctx, Exec> for HNil {
    type ChainOutput = Ctx;

    fn execute(
        self,
        ctx: Ctx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::ChainOutput>> + Send {
        core::future::ready(Ok(ctx))
    }
}

/// Recursive case: run A, feed new ctx HList into B.
impl<A, B, Ctx, Exec, Idx> ExecuteChain<Ctx, Exec> for Then<A, B, Idx>
where
    A: AsyncStep + Send,
    B: AsyncStep + Send,
    Ctx: HList + Sculptor<A::Needs, Idx> + Send,
    Ctx::Remainder: HList + Prepends<A::Provides> + Send,
    <Ctx::Remainder as Prepends<A::Provides>>::Output: HList + Send,
    Exec: ?Sized + Sync + ExecutorFor<A> + ExecutorFor<B>,
    B: ExecuteChain<<Ctx::Remainder as Prepends<A::Provides>>::Output, Exec> + Send,
{
    type ChainOutput = B::ChainOutput;

    fn execute(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::ChainOutput>> + Send {
        let Then(a, b, _) = self;

        async move {
            let new_ctx = a.run_async(ctx, executor).await?;
            b.execute(new_ctx, executor).await
        }
    }
}

/// Lazy step chain bound to an executor type.
pub struct StepChain<Steps: Clone, Exec: ?Sized> {
    pub(crate) steps: Steps,
    _exec: PhantomData<fn() -> Exec>,
}

impl<Steps: Clone, Exec: ?Sized> Clone for StepChain<Steps, Exec> {
    fn clone(&self) -> Self {
        StepChain {
            steps: self.steps.clone(),
            _exec: PhantomData,
        }
    }
}

impl<Exec: ?Sized> StepChain<HNil, Exec> {
    pub fn new() -> Self {
        StepChain {
            steps: HNil,
            _exec: PhantomData,
        }
    }
}

impl<Exec: ?Sized> Default for StepChain<HNil, Exec> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Steps: Clone, Exec: ?Sized + Sync> StepChain<Steps, Exec> {
    pub fn step<S: Clone, Idx: Clone>(self, s: S) -> StepChain<Then<Steps, S, Idx>, Exec>
    where
        Exec: ExecutorFor<S>,
    {
        StepChain {
            steps: Then(self.steps, s, PhantomData),
            _exec: PhantomData,
        }
    }

    pub fn run<H>(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Steps::ChainOutput>> + Send
    where
        Steps: ExecuteChain<H, Exec> + Send,
        H: HList + Send,
    {
        async move { self.steps.execute(ctx, executor).await }
    }
}
