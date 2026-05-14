pub mod auth;
pub mod engine;
pub mod error;
pub mod request;
pub mod stages;
pub mod primitives;


pub trait Command {
    /// The type that the infra layer returns on success.
    type Output;
}

use request::Request;
use stages::{CommandReady, Executed};

use crate::error::PipelineError;


// TODO: GAT?
pub trait CommandExecutor<C: Command>: Send + Sync {
    fn execute(
        &self,
        req: Request<CommandReady, C>,
    ) -> impl std::future::Future<Output = Result<Request<Executed, C::Output>, PipelineError>> + Send;
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
