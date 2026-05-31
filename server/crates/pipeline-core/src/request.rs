use std::marker::PhantomData;

use crate::stages::{Raw, Stage};

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
    pub(crate) fn new(payload: T) -> Self {
        Self {
            payload,
            _stage: PhantomData,
        }
    }

    pub fn wrap(payload: T) -> Self {
        Self::new(payload)
    }

    /// Borrow without consume
    pub fn inner(&self) -> &T {
        &self.payload
    }

    /// Consume, return payload
    pub fn into_inner(self) -> T {
        self.payload
    }

    /// stage transition
    pub(crate) fn advance<NextStage: Stage, U>(self, next: U) -> Request<NextStage, U> {
        Request::new(next)
    }
}
