use std::borrow::Cow;
use crate::formatting::{Color, Decoration};
use crate::identifier::Identifier;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum ClickEvent<'a> {
    OpenUrl(Cow<'a, str>),
    RunCommand(Cow<'a, str>),
    SuggestCommand(Cow<'a, str>),
    ChangePage(usize),
    CopyToClipboard(Cow<'a, str>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum HoverEvent<'a> {
    ShowText(either::Either<Box<TextComponent<'a>>, Cow<'a, str>>),
    ShowItem(Cow<'a, str>),
    ShowEntity(Cow<'a, str>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Component<'a> {
    Text(TextComponent<'a>),
    Translatable(TranslatableComponent<'a>),
    KeyBind(KeyBindComponent<'a>),
    Score(ScoreComponent<'a>),
    Selector(SelectorComponent<'a>),
    Base(BaseComponent<'a>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BaseComponent<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Identifier<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insertion: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "is_cow_empty")]
    extra: Cow<'a, [Component<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_event: Option<ClickEvent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_event: Option<HoverEvent<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent<'a> {
    text: Cow<'a, str>,
    #[serde(flatten)]
    base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TranslatableComponent<'a> {
    translate: Cow<'a, str>,
    #[serde(skip_serializing_if = "is_cow_empty")]
    with: Cow<'a, [Component<'a>]>,
    #[serde(flatten)]
    base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyBindComponent<'a> {
    #[serde(rename = "keybind")]
    key_bind: Cow<'a, str>,
    #[serde(flatten)]
    base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScoreComponent<'a> {
    score: Score<'a>,
    #[serde(flatten)]
    base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Score<'a> {
    name: either::Either<Cow<'a, str>, Uuid>,
    objective: Cow<'a, str>,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectorComponent<'a> {
    selector: Cow<'a, str>,
    #[serde(flatten)]
    base: BaseComponent<'a>,
}

fn is_cow_empty<T: Clone>(value: &Cow<[T]>) -> bool {
    match value {
        Cow::Borrowed(ref data) => data.is_empty(),
        Cow::Owned(ref vec) => vec.is_empty(),
    }
}

impl<'a> BaseComponent<'a> {

    pub fn set_decoration(&mut self, decoration: Decoration, value: Option<bool>) {
        match decoration {
            Decoration::Random => self.obfuscated = value,
            Decoration::Bold => self.bold = value,
            Decoration::Strikethrough => self.strikethrough = value,
            Decoration::Underlined => self.underlined = value,
            Decoration::Italic => self.italic = value,
        }
    }

    pub fn set_color(&mut self, color: Option<impl Into<Color<'a>>>) {
        self.color = color.map(|color| color.into());
    }

    pub fn set_font(&mut self, font: Option<impl Into<Identifier<'a>>>) {
        self.font = font.map(|font| font.into());
    }

    pub fn set_insertion(&mut self, insertion: Option<impl Into<Cow<'a, str>>>) {
        self.insertion = insertion.map(|insertion| insertion.into())
    }

    pub fn set_click_event(&mut self, click_event: Option<impl Into<ClickEvent<'a>>>) {
        self.click_event = click_event.map(|click_event| click_event.into());
    }

    pub fn set_hover_event(&mut self, hover_event: Option<impl Into<HoverEvent<'a>>>) {
        self.hover_event = hover_event.map(|hover_event| hover_event.into());
    }

    pub fn replace_extra(&mut self, extra: impl Into<Cow<'a, [Component<'a>]>>) {
        self.extra = extra.into();
    }

    pub fn add_extra(&mut self, extra: impl Into<Component<'a>>) {
        if let Cow::Borrowed(borrowed) = self.extra {
            self.extra = Cow::Owned(borrowed.to_owned());
        }
        if let Cow::Owned(ref mut owned) = self.extra {
            owned.push(extra.into());
        }
    }

    pub const fn get_bold(&self) -> Option<bool> {
        self.bold
    }

    pub const fn get_italic(&self) -> Option<bool> {
        self.italic
    }

    pub const fn get_underlined(&self) -> Option<bool> {
        self.underlined
    }

    pub const fn get_strikethrough(&self) -> Option<bool> {
        self.strikethrough
    }

    pub const fn get_obfuscated(&self) -> Option<bool> {
        self.obfuscated
    }

    pub const fn get_font(&self) -> &Option<Identifier<'a>> {
        &self.font
    }

    pub const fn get_color(&self) -> &Option<Color<'a>> {
        &self.color
    }

    pub const fn get_insertion(&self) -> &Option<Cow<'a, str>> {
        &self.insertion
    }

    pub const fn get_extra(&self) -> &Cow<'a, [Component<'a>]> {
        &self.extra
    }

    pub const fn get_click_event(&self) -> &Option<ClickEvent<'a>> {
        &self.click_event
    }

    pub const fn get_hover_event(&self) -> &Option<HoverEvent<'a>> {
        &self.hover_event
    }
}