use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

type HexColorInner<'a> = either::Either<(u8, u8, u8), Cow<'a, str>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Random,
    Bold,
    Strikethrough,
    Underlined,
    Italic,
    Reset
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HexColor<'a>(HexColorInner<'a>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Color<'a> {
    Default(DefaultColor),
    Hex(HexColor<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum HexColorError {
    #[error("Hex value contains bad characters")]
    HexValueContainsBadCharacters,
    #[error("Hex value too long")]
    HexValueTooLong,
    #[error("Hex value too small")]
    HexValueTooSmall,
}

impl<'a> HexColor<'a> {
    const fn new(inner: HexColorInner<'a>) -> Self {
        Self(inner)
    }

    const fn get(&self) -> &HexColorInner<'a> {
        &self.0
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(either::Either::Left((r, g, b)))
    }

    pub fn new_hex(hex: impl Into<Cow<'a, str>>) -> Result<Self, HexColorError> {
        let hex = hex.into();
        match hex.len().cmp(&7) {
            Ordering::Less => Err(HexColorError::HexValueTooLong),
            Ordering::Greater => Err(HexColorError::HexValueTooLong),
            // Safety. The length is 7 so next will get first element, which is exist
            Ordering::Equal => match unsafe { hex.chars().next().unwrap_unchecked() } == '#' &&
                hex[1..=7].contains(|c: char| {
                    match c {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => false,
                        _ => true
                    }
                }) {
                true => Err(HexColorError::HexValueContainsBadCharacters),
                false => Ok(Self::new(HexColorInner::Right(hex)))
            }
        }
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self.get() {
            HexColorInner::Left((r, g, b)) => (*r, *g, *b),
            // Safety. Guarantied by private constructors
            HexColorInner::Right(str) => unsafe {
                (
                    u8::from_str_radix(&str[1..3], 16).unwrap_unchecked(),
                    u8::from_str_radix(&str[3..5], 16).unwrap_unchecked(),
                    u8::from_str_radix(&str[5..7], 16).unwrap_unchecked(),
                )
            }
        }
    }

    pub fn get_hex(&'a self) -> Cow<'a, str> {
        match self.get() {
            HexColorInner::Left((r, g, b)) =>
                Cow::Owned(format!("#{:02x}{:02x}{:02x}", r, g, b)),
            HexColorInner::Right(str) => Cow::Borrowed(&str)
        }
    }
}

impl Display for HexColor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_hex())
    }
}

impl TryFrom<String> for HexColor<'_> {
    type Error = HexColorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HexColor::new_hex(value)
    }
}

impl From<HexColor<'_>> for String {
    fn from(color: HexColor<'_>) -> Self {
        color.to_string()
    }
}

impl From<DefaultColor> for Color<'_> {
    fn from(default_color: DefaultColor) -> Self {
        Color::Default(default_color)
    }
}

impl<'a> From<HexColor<'a>> for Color<'a> {
    fn from(hex_color: HexColor<'a>) -> Self {
        Color::Hex(hex_color)
    }
}