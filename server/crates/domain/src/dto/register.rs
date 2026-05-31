use crate::models::{
    Email, HashedPassword, IkPub, IkPubEd, OtpkPub, PlainPassword, SpkPub, SpkPubSig, Username,
};

use pipeline_core::hlist::{HList, IntoHList, hlist};
use pipeline_http::traits::{InfraCommand, IntoCommand};

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

#[derive(Debug)]
pub struct CreatedUser {
    pub id: Uuid,
    pub inserted: i64,
}

impl InfraCommand for RegisterUserCommand {
    type Output = CreatedUser;
}

impl IntoCommand for RegisterBody {
    type Command = RegisterUserCommand;
    fn into_command(self, _identity: &pipeline_http::extractors::auth::Identity) -> Self::Command {
        let hashed_password = self.password.hash();

        RegisterUserCommand {
            username: self.username,
            email: self.email,
            hashed_password,
            ik_pub: self.ik_pub,
            ik_pub_ed: self.ik_pub_ed,
            spk_pub: self.spk_pub,
            spk_pub_sig: self.spk_pub_sig,
            otpks: self.otpks,
        }
    }
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
