use core::ops::Add;

use frunk::hlist::{HList, Sculptor};

use crate::{error::PipelineResult, hlist::ExtendsWith};

pub trait ExecutorFor<S: ?Sized> {}

pub trait SculptRem {
    type Target: HList;
    type Extends: HList;

    type Rem<Ctx, Idx>: HList + ExtendsWith<Self::Target>
    where
        Ctx: HList + Sculptor<Self::Target, Idx>,
        Ctx::Remainder: HList + ExtendsWith<Self::Extends>;
}

pub trait DeriveNewCtx<Ctx, Idx>: SculptRem {
    type NewDerivedCtx: HList;
}

impl<T, Ctx, Idx> DeriveNewCtx<Ctx, Idx> for T
where
    T: SculptRem,
    T::Rem<Ctx, Idx>: HList,
    Ctx: HList + Sculptor<T::Target, Idx>,
    Ctx::Remainder: HList + ExtendsWith<T::Extends>,
{
    type NewDerivedCtx = <T::Rem<Ctx, Idx> as ExtendsWith<Self::Target>>::Output;
}

pub trait PureStep: SculptRem {
    type Needs: HList;
    type Provides: HList;

    fn run_pure<Ctx, Idx>(
        self,
        ctx: Ctx,
    ) -> PipelineResult<<Self as DeriveNewCtx<Ctx, Idx>>::NewDerivedCtx>
    where
        Self: Sized,
        Ctx: HList + Sculptor<Self::Target, Idx>,
        Ctx::Remainder: HList + Add<Self::Extends>,
        <Ctx::Remainder as Add<Self::Extends>>::Output: HList;
}

impl<Exec, S: PureStep> ExecutorFor<S> for Exec {}

pub trait AsyncStep: SculptRem {
    type Needs: HList;
    type Provides: HList;

    fn run_async<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Self as DeriveNewCtx<Ctx, Idx>>::NewDerivedCtx>> + Send
    where
        Self: Sized + Send,
        Self::Rem<Ctx, Idx>: Send,
        <Self::Rem<Ctx, Idx> as ExtendsWith<Self::Target>>::Output: Send,
        Ctx: HList + Sculptor<Self::Target, Idx> + Send,
        Ctx::Remainder: HList + Add<Self::Extends> + Send,
        <Ctx::Remainder as Add<Self::Extends>>::Output: HList + Send,
        Exec: ?Sized + Sync;
}

impl<S: PureStep + Send> AsyncStep for S {
    type Needs = S::Needs;
    type Provides = S::Provides;

    fn run_async<Ctx, Exec, Idx>(
        self,
        ctx: Ctx,
        _executor: &Exec,
    ) -> impl Future<Output = PipelineResult<<Self as DeriveNewCtx<Ctx, Idx>>::NewDerivedCtx>> + Send
    where
        Self: Sized + Send,
        Self::Rem<Ctx, Idx>: Send,
        <Self::Rem<Ctx, Idx> as ExtendsWith<Self::Target>>::Output: Send,
        Ctx: HList + Sculptor<Self::Target, Idx> + Send,
        Ctx::Remainder: HList + Add<Self::Extends> + Send,
        <Ctx::Remainder as Add<Self::Extends>>::Output: HList + Send,
        Exec: ?Sized + Sync,
    {
        core::future::ready(PureStep::run_pure(self, ctx))
    }
}
