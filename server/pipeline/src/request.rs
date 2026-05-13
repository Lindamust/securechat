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
    /// Internal constructor: used only by the transition functions in this crate
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            _stage: PhantomData,
        }
    }

    /// stage transition
    pub fn advance<A, B: Stage>(self, next: A) -> Request<B, A> {
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
}
