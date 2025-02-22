use super::*;

impl StandardErrorDescriptionTrait for PersistentMongoConnectionError {
    fn description(&self) -> Option<&'static str> {
        Some(match self {
            PersistentMongoConnectionError::NoAddress => {""}
            PersistentMongoConnectionError::FailedToEstablishConnection => {""}
        })
    }
}