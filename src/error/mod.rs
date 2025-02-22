use crate::error::connection::PersistentMongoConnectionError;

pub mod connection;

pub enum PersistentMongoError {
    ConnectionError(PersistentMongoConnectionError)
}
