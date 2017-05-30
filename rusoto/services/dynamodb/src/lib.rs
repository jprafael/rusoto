
//! Amazon DynamoDB
//!
//! If you're using the service, you're probably looking for [DynamoDbClient](struct.DynamoDbClient.html) and [DynamoDb](trait.DynamoDb.html).


extern crate futures;
extern crate tokio_core;
extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            