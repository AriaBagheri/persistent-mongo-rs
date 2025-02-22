use super::*;

impl StandardErrorDocsTrait for PersistentMongoConnectionError {
    fn docs(&self) -> &'static str {
        match self {
            PersistentMongoConnectionError::NoAddress => {""}
            PersistentMongoConnectionError::FailedToEstablishConnection => {""}
        }
    }
}