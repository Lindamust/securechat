use domain::dto::{AuthChallengeBody, RegisterBody, SignedTokenBody};
use domain::models::*;
use frunk::HNil;
use frunk::hlist::Plucker;

use core::ops::Add;
use frunk::{HCons, HList, hlist, hlist::HList};

pub trait IntoHList {
    type Output: HList;
    fn into_hlist(self) -> Self::Output;
}

impl IntoHList for RegisterBody {
    type Output = HList![
        Username,
        Email,
        PlainPassword,
        IkPub,
        IkPubEd,
        SpkPub,
        SpkPubSig,
        Vec<OtpkPub>
    ];
    fn into_hlist(self) -> Self::Output {
        hlist![
            self.username,
            self.email,
            self.password,
            self.ik_pub,
            self.ik_pub_ed,
            self.spk_pub,
            self.spk_pub_sig,
            self.otpks
        ]
    }
}

impl IntoHList for AuthChallengeBody {
    type Output = HList![IkPub];
    fn into_hlist(self) -> Self::Output {
        hlist![self.ik_pub]
    }
}

impl IntoHList for SignedTokenBody {
    type Output = HList![IkPub, SigData];
    fn into_hlist(self) -> Self::Output {
        hlist![self.ik_pub, self.sig_data]
    }
}

// copied this code from my hlist-borrow repo  https://github.com/Lindamust/hlist-borrow

pub trait ExtendsWith<S: HList>: HList {
    type Output: HList;
    fn extend_hlist(self, s: S) -> Self::Output;
}

impl<S, H> ExtendsWith<S> for H
where
    S: HList,
    H: HList + core::ops::Add<S>,
    <H as Add<S>>::Output: HList,
{
    type Output = <H as Add<S>>::Output;
    fn extend_hlist(self, s: S) -> Self::Output {
        self + s
    }
}

// wrapper trait for prepending
pub trait Prepends<S>: HList {
    type Output: HList + Send;

    fn prepend_type(self, s: S) -> Self::Output;
}

impl<S: Send> Prepends<S> for HNil {
    type Output = HCons<S, HNil>;

    fn prepend_type(self, s: S) -> Self::Output {
        HCons {
            head: s,
            tail: HNil,
        }
    }
}

impl<S: Send, Head: Send, Tail: HList + Send> Prepends<S> for HCons<Head, Tail> {
    type Output = HCons<S, HCons<Head, Tail>>;

    fn prepend_type(self, s: S) -> Self::Output {
        HCons {
            head: s,
            tail: HCons {
                head: self.head,
                tail: self.tail,
            },
        }
    }
}

// I just copied the Frunk's Sculptor trait but made a few crucial changes that was needed in my program:
// 1. When sculpting out a HNil from a HList, the Idx is still generic instead of HNil.
// Why? because otherwise rust complains that Sculptor<__, HNil> is not implemented when i require Sculptor<__, Idx> for generic Idx
//
// 2. Remainder is send (for async purpose)
//
// ABSOLUTELY FULL CREDIT: https://github.com/lloydmeta/frunk
// (they have open souce MIT license)

pub trait Sculptor<Target, Indices> {
    type Remainder: Send;

    fn sculpt(self) -> (Target, Self::Remainder);
}

/// Implementation for when we have a non-empty HCons target
impl<THead, TTail, SHead, STail, IndexHead, IndexTail>
    Sculptor<HCons<THead, TTail>, HCons<IndexHead, IndexTail>> for HCons<SHead, STail>
where
    HCons<SHead, STail>: Plucker<THead, IndexHead>,
    <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder: Sculptor<TTail, IndexTail>,
{
    type Remainder = <<HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder as Sculptor<
        TTail,
        IndexTail,
    >>::Remainder;

    #[inline(always)]
    fn sculpt(self) -> (HCons<THead, TTail>, Self::Remainder) {
        let (p, r): (
            THead,
            <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder,
        ) = self.pluck();
        let (tail, tail_remainder): (TTail, Self::Remainder) = r.sculpt();
        (HCons { head: p, tail }, tail_remainder)
    }
}

/// remainder inside a single step
pub struct SculptedRemainder<T: HList>(pub T);

impl<T: HList> HList for SculptedRemainder<T> {
    const LEN: usize = T::LEN;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    fn prepend<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self,
        }
    }

    fn static_len() -> usize {
        Self::LEN
    }
}

/// blanket Prepends with NO "T: Prepends<S>" requirement
/// just prepends directly, breaking the normalization chain
impl<T: HList + Send, S: Send> Prepends<S> for SculptedRemainder<T> {
    type Output = HCons<S, T>;
    fn prepend_type(self, s: S) -> Self::Output {
        HCons {
            head: s,
            tail: self.0,
        }
    }
}

/// Blanket Sculptor<HNil, Idx> stays, but Remainder is now SculptedRemainder<Source>
/// instead of Source, preventing the Ctx::Remainder = Ctx normalization
impl<Source: HList + Send, Idx> Sculptor<HNil, Idx> for Source {
    type Remainder = SculptedRemainder<Source>; // <-- THE CHANGE
    #[inline(always)]
    fn sculpt(self) -> (HNil, SculptedRemainder<Source>) {
        (HNil, SculptedRemainder(self))
    }
}
