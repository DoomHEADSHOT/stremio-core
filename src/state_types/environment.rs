use std::error::Error;
use futures::Future;
use serde::de::DeserializeOwned;
pub trait Environment {
    fn fetch_serde<T: 'static>(url: String) -> Box<Future<Item=Box<T>, Error=Box<Error>>> where T: DeserializeOwned;
    // @TODO: get_storage
    // @TODO: set_storage
}
