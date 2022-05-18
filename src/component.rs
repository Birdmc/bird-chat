use serde::{Serialize, Deserialize};
use crate::color::{Color, DefaultColor};
use crate::identifier::Identifier;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(usize),
    CopyToClipboard(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum HoverEvent {
    ShowText(Box<ComponentType>),
    ShowItem(String),
    ShowEntity(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub extra: Vec<ComponentType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextComponent {
    pub text: String,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TranslationComponent {
    pub translate: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub with: Vec<ComponentType>,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyBindComponent {
    pub keybind: String,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScoreComponent {
    pub score: Score,
    #[serde(flatten)]
    pub base: BaseComponent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Score {
    pub name: String,
    pub objective: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

        impl $name {
            pub fn reset(&mut self) {
                self.base.bold = Some(false);
                self.base.italic = Some(false);
                self.base.underlined = Some(false);
                self.base.strikethrough = Some(false);
                self.base.obfuscated = Some(false);
                self.base.color = Some(Color::Default(DefaultColor::Reset));
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

impl TextComponent {
    pub fn new(text: String) -> Self {
        TextComponent {
            text,
            base: BaseComponent::default(),
        }
    }
}

impl TranslationComponent {
    pub fn new(translate: String) -> Self {
        TranslationComponent {
            translate,
            with: Vec::new(),
            base: BaseComponent::default(),
        }
    }
}

impl KeyBindComponent {
    pub fn new(keybind: String) -> Self {
        KeyBindComponent {
            keybind,
            base: BaseComponent::default(),
        }
    }
}

impl ScoreComponent {
    pub fn new(score: Score) -> Self {
        ScoreComponent {
            score,
            base: BaseComponent::default(),
        }
    }
}

impl SelectorComponent {
    pub fn new(selector: String) -> Self {
        SelectorComponent {
            selector,
            base: BaseComponent::default(),
        }
    }
}

impl From<TextComponent> for ComponentType {
    fn from(component: TextComponent) -> Self {
        ComponentType::Text(component)
    }
}

impl From<TranslationComponent> for ComponentType {
    fn from(component: TranslationComponent) -> Self {
        ComponentType::Translation(component)
    }
}

impl From<KeyBindComponent> for ComponentType {
    fn from(component: KeyBindComponent) -> Self {
        ComponentType::KeyBind(component)
    }
}

impl From<ScoreComponent> for ComponentType {
    fn from(component: ScoreComponent) -> Self {
        ComponentType::Score(component)
    }
}

impl From<SelectorComponent> for ComponentType {
    fn from(component: SelectorComponent) -> Self {
        ComponentType::Selector(component)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::HexColor;
    use super::*;

    #[test]
    pub fn color_ser_test() {
        assert_eq!(serde_json::ser::to_string(&Color::Default(DefaultColor::Black)).unwrap(), "\"black\"");
        assert_eq!(serde_json::ser::to_string(&Color::Default(DefaultColor::Aqua)).unwrap(), "\"aqua\"");
        assert_eq!(serde_json::ser::to_string(&Color::Default(DefaultColor::LightPurple)).unwrap(), "\"light_purple\"");
    }

    #[test]
    pub fn color_de_test() {
        assert_eq!(serde_json::de::from_str::<'_, DefaultColor>("\"black\"").unwrap(), DefaultColor::Black);
        assert_eq!(serde_json::de::from_str::<'_, DefaultColor>("\"light_purple\"").unwrap(), DefaultColor::LightPurple);
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

    #[test]
    pub fn component_ser_test() {
        let mut component = TextComponent::new("hello".into());
        component.base.bold = Some(true);
        component.base.color = Some(Color::Default(DefaultColor::Aqua));
        assert_eq!(
            serde_json::ser::to_string(&ComponentType::Text(component.clone())).unwrap(),
            "{\"text\":\"hello\",\"bold\":true,\"color\":\"aqua\"}"
        );
        component.base.color = Some(Color::Hex(HexColor::try_from("ffffff".to_string()).unwrap()));
        assert_eq!(
            serde_json::ser::to_string(&ComponentType::Text(component.clone())).unwrap(),
            "{\"text\":\"hello\",\"bold\":true,\"color\":\"#ffffff\"}"
        );
    }

}