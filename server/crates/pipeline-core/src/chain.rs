use std::future::Future;
use std::marker::PhantomData;

use frunk::{
    HCons, HNil,
    hlist::{HList, Sculptor},
};

use crate::{
    error::PipelineResult,
    step::{AsyncStep, ExecutorFor},
};

// ── Then — composition node ───────────────────────────────────────────────────

#[derive(Clone)]
pub struct Then<A, B, Idx>(pub A, pub B, PhantomData<Idx>);

// ── ExecuteChain — recursive execution trait ──────────────────────────────────

pub trait ExecuteChain<H, Exec: ?Sized> {
    type Output;

    fn execute(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output>> + Send;
}

/// Base case — empty chain, returns context unchanged.
impl<H: HList + Send, Exec: ?Sized> ExecuteChain<H, Exec> for HNil {
    type Output = H;

    fn execute(
        self,
        ctx: H,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output>> + Send {
        async move { Ok(ctx) }
    }
}

/// Recursive case — run A, feed extended HList into B.
impl<H, Idx, A, B, Exec> ExecuteChain<H, Exec> for Then<A, B, Idx>
where
    A: AsyncStep + Send,
    H: HList + Sculptor<A::Needs, Idx> + Send,
    Exec: ExecutorFor<A> + ?Sized + Sync,
    B: ExecuteChain<HCons<A::Provides, H>, Exec> + Send,
{
    type Output = B::Output;

    fn execute(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output>> + Send {
        async move {
            let mid = self.0.run(ctx, executor).await?;
            self.1.execute(mid, executor).await
        }
    }
}

// ── StepChain ─────────────────────────────────────────────────────────────────

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
    ) -> impl Future<Output = PipelineResult<Steps::Output>> + Send
    where
        Steps: ExecuteChain<H, Exec> + Send,
        H: HList + Send,
    {
        async move { self.steps.execute(ctx, executor).await }
    }
}
