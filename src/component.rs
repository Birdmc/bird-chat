use std::borrow::Cow;
use crate::formatting::Color;
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