use super::*;

impl StandardErrorCodeTrait for PersistentMongoConnectionError {
    fn code(&self) -> usize {
        match self {
            PersistentMongoConnectionError::NoAddress => {0}
            PersistentMongoConnectionError::FailedToEstablishConnection => {1}
        }
    }
}