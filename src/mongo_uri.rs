use std::str::FromStr;

pub struct MongoUri(String);

impl MongoUri {
    pub fn from_str(value: String) -> MongoUri {
        MongoUri(value)
    }
}

impl FromStr for MongoUri {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}