use std::marker::PhantomData;

use super::{
    error::PipelineResult,
    stages::{Raw, Stage},
};

pub struct Request<S: Stage, T> {
    pub(crate) payload: T,
    _stage: PhantomData<S>,
}

impl<T> Request<Raw, T> {
    pub fn from_raw(payload: T) -> Self {
        Self::new(payload)
    }
}

impl<S: Stage, T> Request<S, T> {
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            _stage: PhantomData,
        }
    }

    /// stage transition
    pub fn advance<U, B: Stage>(self, next: U) -> Request<B, U> {
        Request::new(next)
    }

    /// Borrow without consume
    pub fn inner(&self) -> &T {
        &self.payload
    }

    /// Consume, return payload
    pub fn into_inner(self) -> T {
        self.payload
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Request<S, U> {
        Request::new(f(self.into_inner()))
    }

    pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Request<S, U>, E> {
        Ok(Request::new(f(self.into_inner())?))
    }

    pub fn advance_with<U, B: Stage>(self, f: impl FnOnce(T) -> U) -> Request<B, U> {
        Request::new(f(self.into_inner()))
    }

    pub fn try_advance_with<U, B: Stage, E>(
        self,
        f: impl FnOnce(T) -> Result<U, E>,
    ) -> Result<Request<B, U>, E> {
        Ok(Request::new(f(self.into_inner())?))
    }
}

pub trait Transition<NextStage: Stage, Output> {
    fn try_advance(self) -> PipelineResult<Request<NextStage, Output>>;
}
