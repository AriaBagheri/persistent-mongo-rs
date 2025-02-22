use super::*;

impl StandardErrorSolutionsTrait for PersistentMongoConnectionError {
    fn solutions(&self) -> Option<&'static str> {
        Some(match self {
            PersistentMongoConnectionError::NoAddress => {""}
            PersistentMongoConnectionError::FailedToEstablishConnection => {""}
        })
    }
}