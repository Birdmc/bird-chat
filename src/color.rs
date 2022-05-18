use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DefaultColor {
    // colors
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    // styles
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String", into = "String")]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub enum HexColorError {
    BadHex
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Color {
    Default(DefaultColor),
    Hex(HexColor),
}

impl Display for HexColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HexColorError::BadHex => write!(f, "Bad hex string"),
        }
    }
}

impl TryFrom<String> for HexColor {
    type Error = HexColorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_hex_num = i32::from_str_radix(&*value, 16)
            .map_err(|_| HexColorError::BadHex)?;
        Ok(HexColor {
            r: (value_hex_num >> 16 & 0xff) as u8,
            g: (value_hex_num >> 8 & 0xff) as u8,
            b: (value_hex_num & 0xff) as u8,
        })
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl From<HexColor> for String {
    fn from(color: HexColor) -> Self {
        color.to_string()
    }
}

impl From<DefaultColor> for Color {
    fn from(color: DefaultColor) -> Self {
        Color::Default(color)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::HexColor;

    #[test]
    fn success_hex_color_test() {
        let hex_color = HexColor::try_from("0f0f0f".to_string()).unwrap();
        assert_eq!(hex_color.r, 15);
        assert_eq!(hex_color.g, 15);
        assert_eq!(hex_color.b, 15);
        assert_eq!(hex_color.to_string(), "0f0f0f");
    }
}