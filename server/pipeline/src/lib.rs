pub mod auth;
pub mod engine;
pub mod error;
pub mod request;
pub mod stages;

pub trait Command {
    /// The type that the infra layer returns on success.
    type Output;
}

use error::PipelineResult;
use request::Request;
use stages::{CommandReady, Executed};

pub trait CommandExecutor<C: Command> {
    async fn execute(
        &self,
        req: Request<CommandReady, C>,
    ) -> PipelineResult<Request<Executed, C::Output>>;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
