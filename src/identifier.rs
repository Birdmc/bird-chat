use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
enum IdentifierInner<'a> {
    Fulled(&'a str),
    Partial(&'a str, &'a str),
}

#[derive(Clone, Debug)]
pub struct Identifier<'a>(IdentifierInner<'a>);

impl<'a> Identifier<'a> {
    const fn new(inner: IdentifierInner<'a>) -> Self {
        Self(inner)
    }

    const fn get_inner(&self) -> &IdentifierInner<'a> {
        &self.0
    }

    /// If value is full identifier then new_fulled function with value argument will be used
    /// If value is not full identifier then new_partial with default_key and value arguments will be used
    pub fn new_with_default(value: &'a str, default_key: &'a str) -> Option<Self> {
        match value.find(":") {
            // Safety. as full contains ':' then this function will always return Some
            Some(index) => match unsafe { value.rfind(":").unwrap_unchecked() } == index {
                true => Some(Self::new(IdentifierInner::Fulled(value))),
                false => None,
            },
            None => match default_key.contains(":") {
                true => None,
                false => Some(Self::new(IdentifierInner::Partial(default_key, value)))
            }
        }
    }

    pub fn new_fulled(full: &'a str) -> Option<Self> {
        match full.find(":") {
            // Safety. as full contains ':' then this function will always return Some
            Some(index) => match unsafe { full.rfind(":").unwrap_unchecked() } == index {
                true => Some(Self::new(IdentifierInner::Fulled(full))),
                false => None,
            },
            None => None,
        }
    }

    pub fn new_partial(key: &'a str, value: &'a str) -> Option<Self> {
        match key.contains(":") || value.contains(":") {
            true => None,
            false => Some(Self::new(IdentifierInner::Partial(key, value)))
        }
    }

    pub fn get_fulled(&self) -> Cow<'a, str> {
        match self.get_inner() {
            IdentifierInner::Fulled(str) => Cow::Borrowed(str),
            IdentifierInner::Partial(key, value) => Cow::Owned(format!("{}:{}", key, value))
        }
    }

    pub fn get_partial(&self) -> (&'a str, &'a str) {
        match self.get_inner() {
            IdentifierInner::Fulled(str) => {
                let index = unsafe { str.find(":").unwrap_unchecked() };
                (&str[0..index], &str[index + 1..str.len()])
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