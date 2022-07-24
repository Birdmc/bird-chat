use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::pattern::{Pattern, Searcher};

#[derive(Clone, Debug)]
pub enum IdentifierInner<'a> {
    Fulled(Cow<'a, str>),
    Partial(Cow<'a, str>, Cow<'a, str>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
#[repr(transparent)]
pub struct Identifier<'a>(IdentifierInner<'a>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, thiserror::Error)]
pub enum IdentifierError {
    #[error("Value contains double dot")]
    ValueContainsDoubleDot,
    #[error("Key contains double dot")]
    KeyContainsDoubleDot,
    #[error("Fulled contains more than one double dot")]
    FulledContainsMoreThanOneDoubleDot,
    #[error("Fulled contains no double dots")]
    FulledContainsNoDoubleDot,
}

impl<'a> Identifier<'a> {
    const fn new(inner: IdentifierInner<'a>) -> Self {
        Self(inner)
    }

    const fn get_inner(&self) -> &IdentifierInner<'a> {
        &self.0
    }

    pub fn into_inner(self) -> IdentifierInner<'a> {
        self.0
    }

    pub const unsafe fn from_inner_unchecked(inner: IdentifierInner<'a>) -> Self {
        Self::new(inner)
    }

    pub fn from_inner(inner: IdentifierInner<'a>) -> Result<Self, IdentifierError> {
        match inner {
            IdentifierInner::Fulled(fulled) => Self::new_fulled(fulled),
            IdentifierInner::Partial(key, value) => Self::new_partial(key, value),
        }
    }

    /// If value is full identifier then new_fulled function with value argument will be used.
    /// If value is not full identifier then new_partial with default_key and value arguments will be used
    pub fn new_with_default(value: impl Into<Cow<'a, str>>, default_key: impl Into<Cow<'a, str>>) -> Result<Self, IdentifierError> {
        let value = value.into();
        let default_key = default_key.into();
        let mut searcher = ':'.into_searcher(&value);
        match searcher.next_match() {
            Some(_) => match searcher.next_match() {
                Some(_) => Err(IdentifierError::FulledContainsMoreThanOneDoubleDot),
                None => Ok(Self::new(IdentifierInner::Fulled(value))),
            },
            None => match default_key.contains(':') {
                true => Err(IdentifierError::KeyContainsDoubleDot),
                false => Ok(Self::new(IdentifierInner::Partial(default_key, value)))
            }
        }
    }

    pub fn new_fulled(full: impl Into<Cow<'a, str>>) -> Result<Self, IdentifierError> {
        let full = full.into();
        let mut searcher = ':'.into_searcher(&full);
        match searcher.next_match() {
            Some(_) => match searcher.next_match() {
                Some(_) => Err(IdentifierError::FulledContainsMoreThanOneDoubleDot),
                None => Ok(Self::new(IdentifierInner::Fulled(full))),
            },
            None => Err(IdentifierError::FulledContainsNoDoubleDot),
        }
    }

    pub fn new_partial(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Result<Self, IdentifierError> {
        let key = key.into();
        let value = value.into();
        match key.contains(':') {
            true => Err(IdentifierError::KeyContainsDoubleDot),
            false => match value.contains(':') {
                true => Err(IdentifierError::ValueContainsDoubleDot),
                false => Ok(Self::new(IdentifierInner::Partial(key, value)))
            }
        }
    }

    pub fn get_fulled(&'a self) -> Cow<'a, str> {
        match self.get_inner() {
            IdentifierInner::Fulled(fulled) => Cow::Borrowed(&fulled),
            IdentifierInner::Partial(key, value) =>
                Cow::Owned(format!("{}:{}", key, value))
        }
    }

    pub fn get_partial(&'a self) -> (&'a str, &'a str) {
        match self.get_inner() {
            IdentifierInner::Fulled(fulled) => {
                // Safety. guarantied by constructors
                let index = unsafe { fulled.find(':').unwrap_unchecked() };
                (&fulled[0..index], &fulled[index + 1..fulled.len()])
            }
            IdentifierInner::Partial(key, value) => (&key, &value)
        }
    }

    pub fn into_fulled(self) -> Cow<'a, str> {
        match self.into_inner() {
            IdentifierInner::Fulled(fulled) => fulled,
            IdentifierInner::Partial(key, value) =>
                Cow::Owned(format!("{}:{}", key, value))
        }
    }

    pub fn into_partial(self) -> (Cow<'a, str>, Cow<'a, str>) {
        match self.into_inner() {
            IdentifierInner::Fulled(fulled) => {
                // Safety. guarantied by constructors
                let index = unsafe { fulled.find(':').unwrap_unchecked() };
                (
                    Cow::Owned(fulled[0..index].to_owned()),
                    Cow::Owned(fulled[index + 1..fulled.len()].to_owned())
                )
            }
            IdentifierInner::Partial(key, value) => (key, value)
        }
    }

    pub const fn is_fulled(&self) -> bool {
        match self.get_inner() {
            IdentifierInner::Fulled(_) => true,
            IdentifierInner::Partial(..) => false,
        }
    }

    pub const fn is_partial(&self) -> bool {
        !self.is_fulled()
    }
}

impl Display for IdentifierInner<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fulled(fulled) => write!(f, "{}", fulled),
            Self::Partial(key, value) => write!(f, "{}:{}", key, value)
        }
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_inner())
    }
}

impl PartialEq for Identifier<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_partial() == other.get_partial()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl TryFrom<String> for Identifier<'_> {
    type Error = IdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new_fulled(value)
    }
}

impl From<Identifier<'_>> for String {
    fn from(identifier: Identifier<'_>) -> Self {
        identifier.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn identifier_constructor() {
        {
            let good_identifier = Identifier::new_fulled("minecraft:grass_block").unwrap();
            assert_eq!(good_identifier.to_string(), "minecraft:grass_block");
            assert_eq!(good_identifier.get_partial(), ("minecraft", "grass_block"));
            assert_eq!(good_identifier.get_fulled(), Cow::Borrowed("minecraft:grass_block"));
            assert_eq!(
                good_identifier,
                Identifier::from_inner(IdentifierInner::Fulled(Cow::Borrowed("minecraft:grass_block"))).unwrap()
            );
        }
        {
            assert_eq!(
                Identifier::new_partial("minecraft", "grass_block"),
                Identifier::new_fulled("minecraft:grass_block")
            );
        }
        {
            assert_eq!(
                Identifier::new_fulled("minecraft:grass_block:"),
                Err(IdentifierError::FulledContainsMoreThanOneDoubleDot)
            );
            assert_eq!(
                Identifier::new_fulled("minecraft_grass_block"),
                Err(IdentifierError::FulledContainsNoDoubleDot)
            );
            assert_eq!(
                Identifier::new_partial("minecraft", "grass_block:"),
                Err(IdentifierError::ValueContainsDoubleDot)
            );
            assert_eq!(
                Identifier::new_partial("minecraft:", "grass_block"),
                Err(IdentifierError::KeyContainsDoubleDot)
            );
        }
        {
            assert_eq!(
                Identifier::new_with_default("grass_block", "minecraft"),
                Identifier::new_fulled("minecraft:grass_block")
            );
            assert_eq!(
                Identifier::new_with_default("other:grass_block", "minecraft"),
                Identifier::new_fulled("other:grass_block")
            );
        }
    }

    #[test]
    fn into() {
        {
            let identifier = Identifier::new_fulled("minecraft:grass_block").unwrap();
            assert_eq!(identifier.get_fulled(), Cow::Borrowed("minecraft:grass_block"));
            assert_eq!(identifier.get_partial(), ("minecraft", "grass_block"));
        }
        {
            let identifier = Identifier::new_partial("minecraft", "grass_block").unwrap();
            assert_eq!(identifier.get_fulled(), Cow::Owned::<str>("minecraft:grass_block".to_string()));
            assert_eq!(identifier.get_partial(), ("minecraft", "grass_block"));
        }
    }
}