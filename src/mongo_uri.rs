use std::str::FromStr;

pub struct MongoUri(String);

impl FromStr for MongoUri {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}