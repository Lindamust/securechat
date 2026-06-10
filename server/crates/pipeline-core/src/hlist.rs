use domain::dto::{AuthChallengeBody, RegisterBody, SignedTokenBody};
use domain::models::*;

use core::ops::Add;
use frunk::{HList, hlist, hlist::HList};

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
