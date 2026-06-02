use crate::{error::HttpResult, extractors::auth::Identity};

use pipeline_core::{request::Request, stages::Executed};

pub trait InfraCommand {
    /// The type that the infra layer returns on success.
    type Output;
}

// "Missing Required Bounds error"
// https://github.com/rust-lang/rust/issues/87479
pub trait CommandExecutor<C: InfraCommand> {
    fn execute(
        &self,
        cmd: C,
    ) -> impl Future<Output = HttpResult<Request<Executed, C::Output>>> + Send;
}

pub trait IntoCommand {
    type Command: InfraCommand;
    fn into_command(self, identity: &Identity) -> Self::Command;
}

// Trait Extention Implementations

// Auth - Challenge
use domain::dto::{AuthChallengeBody, InsertedNonce};

impl InfraCommand for AuthChallengeBody {
    type Output = InsertedNonce;
}

impl IntoCommand for AuthChallengeBody {
    type Command = Self;
    fn into_command(self, _idenity: &crate::extractors::auth::Identity) -> Self::Command {
        self
    }
}

// Register
use domain::dto::{InsertedUser, RegisterBody, RegisterUserCommand};

impl InfraCommand for RegisterUserCommand {
    type Output = InsertedUser;
}

impl IntoCommand for RegisterBody {
    type Command = RegisterUserCommand;
    fn into_command(self, _identity: &crate::extractors::auth::Identity) -> Self::Command {
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
