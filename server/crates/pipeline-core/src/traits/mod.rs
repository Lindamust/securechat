use crate::{
    extractors::auth::Identity,
    typestate::{error::PipelineResult, request::Request, stages::Executed},
};

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
    ) -> impl Future<Output = PipelineResult<Request<Executed, C::Output>>> + Send;
}

pub trait IntoCommand {
    type Command: InfraCommand;
    fn into_command(self, idenity: &Identity) -> Self::Command;
}
