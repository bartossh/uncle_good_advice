// Abstractions that settle all the contracts in the uncle_good_advice library.
// Particular types in the library implement the traits so the dependencies are loosely coupled
// and can be echanged by any other implementation given by the user.;
use serde::ser::Serialize;
use serde::Deserialize;
use std::future::Future;
use std::u128;

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

/// Store requires from entity to have storage capability for the entity. It might be a permanent storage or any form of cache.
pub trait Store<'a, T>
where
    T: Serialize + Deserialize<'a> + Send,
{
    /// Saves serializeble entity in to the storage.
    ///
    /// # Arguments
    ///
    /// * `entity` - serializable entity to be stored.
    ///
    /// # Returns
    ///
    /// * Future with Success `String` os saved entity ID or Error `String` with message about failure.
    fn save(&self, entity: &T) -> impl Future<Output = Result<String, String>>;

    /// Read by id serializeble entity from the storage.
    ///
    /// # Arguments
    ///
    /// * `id` - unique id of the entity.
    ///
    /// # Returns
    ///
    /// * Future with Success entity of type T from storage
    /// or Error `String` with message about failure.
    fn read_by_id(&self, id: &str) -> impl Future<Output = Result<T, String>>;

    /// Reads entities from given time from the storage.
    ///
    /// # Arguments
    ///
    /// * `timestampms - inclusive timestamp in [ ms ] from which to read entities.
    ///
    /// # Returns
    ///
    /// * Future with Success `Vec<T> with vector of type `T` entities
    /// or Error `String` with message about failure.
    fn read_from_time(&self, timestamp_ms: u128) -> impl Future<Output = Result<Vec<T>, String>>;
}

/// Fetcher requires from entity to have fetching capability. It shall fetch data from external resource.
pub trait Fetcher<'a, T>
where
    T: Serialize + Deserialize<'a> + Send,
{
    /// Pulls data from external resource.
    ///
    /// # Returns
    ///
    /// * Future with Success `Vec<T>` entities from the source
    /// or Error `String` with message about failure.
    fn pull(&self) -> impl Future<Output = Result<Vec<T>, String>> + Send;
}

/// FilterStr requires from entity to have capabilities of filtering things from str buffer and returning them as slice of `T` type.
pub trait FilterStr<'a, T>
where
    T: Serialize + Deserialize<'a>,
{
    /// Filters data from str.
    ///
    /// # Returns
    ///
    /// * `Vec<T>` of all filtered entities from the str.
    fn filter_str(&self, text: &str) -> Vec<T>;
}

/// ValidatorStrategy requires from entity to provide validation strategy for `T`.
pub trait ValidatorStrategy<T> {
    /// Analyzes if value is valid under the specific strategy.
    ///
    /// * `value` - Value to be analyzed under specific strategy implementation.
    ///
    /// # Returns
    ///
    /// * Success `true` if is valid or `false` otherwise.
    fn is_valid(&self, value: &T) -> bool;
}

/// ExtractionStrategy requires from entity to extract values `E` from type `T`.
pub trait ExtractionStrategy<T, E> {
    /// Extracts values `E` from type `T`.
    ///
    /// * `data` - Data type that will be used for values `E` extraction from it.
    ///
    /// # Returns
    ///
    /// * Vector of `E` data types extracted from given type `T`.
    fn extract(&self, data: &T) -> Vec<E>;
}
