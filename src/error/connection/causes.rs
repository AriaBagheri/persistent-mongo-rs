use super::*;

impl StandardErrorCausesTrait for PersistentMongoConnectionError {
    fn causes(&self) -> Option<&'static str> {
        Some(match self {
            PersistentMongoConnectionError::NoAddress => {""}
            PersistentMongoConnectionError::FailedToEstablishConnection => {""}
        })
    }
}