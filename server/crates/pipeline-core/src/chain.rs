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

// Then composition node

#[derive(Clone)]
pub struct Then<A, B, Idx>(pub A, pub B, PhantomData<Idx>);

// ExecuteChain recursive execution trait

pub trait ExecuteChain<H, Exec: ?Sized> {
    type Output;

    fn execute(
        self,
        ctx: H,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output>> + Send;
}

/// Base case: empty chain, returns context unchanged.
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

/// Recursive case: run A, feed new ctx HList into B.
//
// H1 = Input Ctx for A
// H2 = Output Ctx of A and Input Ctx for B
// H3 = Total output
// Idx is type inferred for each (?)
// A: first step
// B: second step
// Exec: The runtime executing each step
impl<H1, Idx, A, B, Exec> ExecuteChain<H1, Exec> for Then<A, B, Idx>
where
    A: AsyncStep + Send,
    B: AsyncStep + Send,

    // Frunk Sculptor and HList
    H1: HList + Sculptor<A::Needs, Idx> + Send,
    A::Output<H1>: HList + Sculptor<B::Needs, Idx> + Send,
    B::Output<A::Output<H1>>: HList + Send,

    Exec: ExecutorFor<A> + ExecutorFor<B> + ?Sized + Sync,
{
    type Output = B::Output<A::Output<H1>>;

    fn execute(
        self,
        ctx: H1,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<Self::Output>> + Send {
        async move {
            let Then(a, b, _) = self;

            let ctx2= a.run(ctx, executor).await?;
            let ctx3 = b.run(ctx2, executor).await?;

            Ok(ctx3)
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
