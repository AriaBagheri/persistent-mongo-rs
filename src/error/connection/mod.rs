use std::error::Error;

mod causes;
mod docs;
mod description;
mod solutions;
mod code;

use standard_error::traits::*;

pub enum PersistentMongoConnectionError {
    NoAddress,
    FailedToEstablishConnection
}