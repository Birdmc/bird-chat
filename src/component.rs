use serde::{Serialize, Deserialize};
use crate::identifier::Identifier;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    // colors
    Black, DarkBlue, DarkGreen, DarkAqua,
    DarkRed, DarkPurple, Gold, Gray,
    DarkGray, Blue, Green, Aqua,
    Red, LightPurple, Yellow, White,
    // styles
    Obfuscated, Bold, Strikethrough,
    Underline, Italic, Reset,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(usize),
    CopyToClipboard(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum HoverEvent {
    ShowText(Box<ComponentType>),
    ShowItem(String),
    ShowEntity(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ComponentType {
    Text(TextComponent),
    Translation(TranslationComponent),
    KeyBind(KeyBindComponent),
    Score(ScoreComponent),
    Selector(SelectorComponent),
}

pub trait Component {
    fn get_base(&self) -> &BaseComponent;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseComponent {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub font: Option<Identifier>,
    pub color: Option<Color>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub extras: Vec<ComponentType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextComponent {
    pub text: String,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationComponent {
    pub translate: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub with: Vec<ComponentType>,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindComponent {
    pub keybind: String,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreComponent {
    pub score: Score,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub name: String,
    pub objective: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectorComponent {
    pub selector: String,
    #[serde(flatten)]
    pub base: BaseComponent,
}

macro_rules! component {
    ($name: ident) => {
        impl Component for $name {
            fn get_base(&self) -> &BaseComponent {
                &self.base
            }
        }
    };
    ($($name: ident)*) => {
        $(component!($name);)*
    }
}

impl Component for BaseComponent {
    fn get_base(&self) -> &BaseComponent {
        self
    }
}

component!(TextComponent TranslationComponent KeyBindComponent ScoreComponent SelectorComponent);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn color_ser_test() {
        assert_eq!(serde_json::ser::to_string(&Color::Black).unwrap(), "\"black\"");
        assert_eq!(serde_json::ser::to_string(&Color::Aqua).unwrap(), "\"aqua\"");
        assert_eq!(serde_json::ser::to_string(&Color::LightPurple).unwrap(), "\"light_purple\"");
    }

    #[test]
    pub fn color_de_test() {
        assert_eq!(serde_json::de::from_str::<'_, Color>("\"black\"").unwrap(), Color::Black);
        assert_eq!(serde_json::de::from_str::<'_, Color>("\"light_purple\"").unwrap(), Color::LightPurple);
    }

    #[test]
    pub fn click_event_ser_test() {
        assert_eq!(
            serde_json::ser::to_string(&ClickEvent::OpenUrl("http://google.com".into())).unwrap(),
            "{\"action\":\"open_url\",\"value\":\"http://google.com\"}"
        );
        assert_eq!(
            serde_json::ser::to_string(&ClickEvent::ChangePage(100)).unwrap(),
            "{\"action\":\"change_page\",\"value\":100}"
        )
    }

}