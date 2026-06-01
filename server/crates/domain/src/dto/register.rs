use crate::models::{
    Email, HashedPassword, IkPub, IkPubEd, OtpkPub, PlainPassword, SpkPub, SpkPubSig, Username,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// HTTP request + response body

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterBody {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub inserted: i64,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: Username,
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: Username,
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

#[derive(Debug, Clone)]
pub struct InsertedUser {
    pub id: Uuid,
    pub inserted_otpks: i64,
}

// impl IntoHList for RegisterBody {
//     type Output = HList![
//         Username,
//         Email,
//         PlainPassword,
//         IkPub,
//         IkPubEd,
//         SpkPub,
//         SpkPubSig,
//         Vec<OtpkPub>
//     ];
//     fn into_hlist(self) -> Self::Output {
//         hlist![
//             self.username,
//             self.email,
//             self.password,
//             self.ik_pub,
//             self.ik_pub_ed,
//             self.spk_pub,
//             self.spk_pub_sig,
//             self.otpks
//         ]
//     }
// }
