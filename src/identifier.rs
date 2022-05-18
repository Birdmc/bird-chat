use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::identifier::Error::{NoAnyDoubleDots, TooManyDoubleDots};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "String", into = "String")]
pub struct Identifier {
    domain: String,
    key: String,
    full: String,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TooManyDoubleDots,
    NoAnyDoubleDots,
}

impl Identifier {
    pub fn from_parts(domain: String, key: String) -> Result<Identifier, Error> {
        match domain.contains(':') || key.contains(':') {
            true => Err(TooManyDoubleDots),
            false => {
                let full = format!("{}:{}", domain, key);
                Ok(Identifier { domain, key, full })
            }
        }
    }

    pub fn from_full(full: String) -> Result<Identifier, Error> {
        match full.find(':') {
            Some(index) => match full.rfind(':') {
                Some(rindex) => match index == rindex {
                    true => Ok(Identifier {
                        domain: full[0..index].to_owned(),
                        key: full[index+1..].to_owned(),
                        full,
                    }),
                    false => Err(TooManyDoubleDots),
                },
                None => unreachable!(),
            },
            None => Err(NoAnyDoubleDots)
        }
    }

    pub fn get_domain(&self) -> &String {
        &self.domain
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_full(&self) -> &String {
        &self.domain
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full)
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.to_string()
    }
}

impl TryFrom<String> for Identifier {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Identifier::from_full(value)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooManyDoubleDots => write!(f, "Too many double dots"),
            Error::NoAnyDoubleDots => write!(f, "No any double dots"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn success_from_parts_test() {
        let identifier = Identifier::from_parts("minecraft".into(), "grass_block".into()).unwrap();
        assert_eq!(identifier.get_domain(), "minecraft");
        assert_eq!(identifier.get_key(), "grass_block");
    }

    #[test]
    pub fn too_many_double_dots_from_parts_test() {
        assert_eq!(
            Identifier::from_parts("minecraft:".into(), "some".into()).unwrap_err(),
            Error::TooManyDoubleDots
        );
        assert_eq!(
            Identifier::from_parts("minecraft".into(), "some:".into()).unwrap_err(),
            Error::TooManyDoubleDots
        );
    }

    #[test]
    pub fn success_from_full_test() {
        let identifier = Identifier::from_full("minecraft:grass_block".into()).unwrap();
        assert_eq!(identifier.get_domain(), "minecraft");
        assert_eq!(identifier.get_key(), "grass_block");
    }

    #[test]
    pub fn too_many_double_dots_from_full_test() {
        assert_eq!(
            Identifier::from_full("minecraft::grass_block".into()).unwrap_err(),
            Error::TooManyDoubleDots
        );
    }

    #[test]
    pub fn no_any_double_dots_from_full_test() {
        assert_eq!(
            Identifier::from_full("minecraft_grass_block".into()).unwrap_err(),
            Error::NoAnyDoubleDots
        )
    }

}