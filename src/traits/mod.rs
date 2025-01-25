// Abstractions that settle all the contracts in the uncle_good_advice library.
// Particular types in the library implement the traits so the dependencies are loosely coupled
// and can be echanged by any other implementation given by the user.
use std::future::Future;

/// Configure requires from entity to be able to return required configuration.
pub trait Configur {
    /// Prompt from configuration that is a sentence to create a specific model.
    ///
    /// # Returns
    ///
    /// * String with prompt message.
    fn prompt(&self) -> String;
}

/// Advise requires from entity to be able to advise and analyze by responding to given message.
pub trait Advise<'a> {
    /// Analyzes the message and responds with message containing the analyze result.
    /// The response message is an human readable text and can be about anything.
    /// Response is dependent totally on the entity capabilitties and setup such as prompt or else.
    ///
    /// * `msg` - Message to be analyzed  by the entity.
    ///
    /// # Returns
    ///
    /// * Future with Success `String` that is an analyze result or Error `String` with message about failure.
    fn advise_about(&mut self, msg: &str) -> impl Future<Output = Result<String, String>> + Send;
}

/// Handler requires from entity to handle the process of the whole program. This can be server, cli tool or cron runner.
pub trait Handler {
    /// Runs the handler in async manner.
    ///
    /// # Returns
    ///
    /// * Future with Success `()` if runner runs without issue or Error `String` with message about failure.
    fn run(&self) -> impl Future<Output = Result<(), String>>;
}
