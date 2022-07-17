use std::borrow::Cow;
use crate::formatting::{Color};
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
    pub font: Option<Identifier<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "is_cow_empty")]
    pub extra: Cow<'a, [Component<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent<'a> {
    pub text: Cow<'a, str>,
    #[serde(flatten)]
    pub base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TranslatableComponent<'a> {
    pub translate: Cow<'a, str>,
    #[serde(skip_serializing_if = "is_cow_empty")]
    pub with: Cow<'a, [Component<'a>]>,
    #[serde(flatten)]
    pub base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyBindComponent<'a> {
    #[serde(rename = "keybind")]
    pub key_bind: Cow<'a, str>,
    #[serde(flatten)]
    pub base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScoreComponent<'a> {
    pub score: Score<'a>,
    #[serde(flatten)]
    pub base: BaseComponent<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Score<'a> {
    pub name: either::Either<Cow<'a, str>, Uuid>,
    pub objective: Cow<'a, str>,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectorComponent<'a> {
    pub selector: Cow<'a, str>,
    #[serde(flatten)]
    pub base: BaseComponent<'a>,
}

fn is_cow_empty<T: Clone>(value: &Cow<[T]>) -> bool {
    match value {
        Cow::Borrowed(ref data) => data.is_empty(),
        Cow::Owned(ref vec) => vec.is_empty(),
    }
}

fn make_owned<T: ToOwned + ?Sized>(cow: &mut Cow<T>) {
    if let Cow::Borrowed(borrowed) = cow {
        *cow = Cow::Owned(borrowed.to_owned())
    }
}

fn add<T: ToOwned + Clone>(into: &mut Cow<[T]>, to_add: T) {
    match into.is_empty() {
        true => *into = Cow::Owned(vec![to_add]),
        false => {
            make_owned(into);
            match into {
                Cow::Owned(ref mut owned) => owned.push(to_add),
                // Safety. guarantied by make_owned
                _ => unsafe { std::hint::unreachable_unchecked() }
            }
        }
    }
}

fn add_values<'a, T: ToOwned + Clone>(into: &mut Cow<'a, [T]>, to_add: Cow<'a, [T]>) {
    match into.is_empty() {
        true => *into = to_add,
        false => {
            make_owned(into);
            match into {
                Cow::Owned(ref mut owned) => {
                    let mut to_add = to_add.into();
                    make_owned(&mut to_add);
                    match to_add {
                        Cow::Owned(push) => for to_add in push {
                            owned.push(to_add)
                        },
                        // Safety. guarantied by make_owned
                        _ => unsafe { std::hint::unreachable_unchecked() }
                    }
                }
                // Safety. guarantied by make_owned
                _ => unsafe { std::hint::unreachable_unchecked() }
            }
        }
    }
}

impl<'a> BaseComponent<'a> {
    pub fn add_extra(&mut self, extra: impl Into<Component<'a>>) {
        add(&mut self.extra, extra.into())
    }

    pub fn add_extras(&mut self, extras: impl Into<Cow<'a, [Component<'a>]>>) {
        add_values(&mut self.extra, extras.into());
    }
}

impl <'a> TranslatableComponent<'a> {
    pub fn add_arg(&mut self, arg: impl Into<Component<'a>>) {
        add(&mut self.with, arg.into())
    }

    pub fn add_args(&mut self, args: impl Into<Cow<'a, [Component<'a>]>>) {
        add_values(&mut self.with, args.into());
    }
}

impl<'a> From<TextComponent<'a>> for Component<'a> {
    fn from(component: TextComponent<'a>) -> Self {
        Self::Text(component)
    }
}

impl<'a> From<TranslatableComponent<'a>> for Component<'a> {
    fn from(component: TranslatableComponent<'a>) -> Self {
        Self::Translatable(component)
    }
}

impl<'a> From<ScoreComponent<'a>> for Component<'a> {
    fn from(component: ScoreComponent<'a>) -> Self {
        Self::Score(component)
    }
}

impl<'a> From<SelectorComponent<'a>> for Component<'a> {
    fn from(component: SelectorComponent<'a>) -> Self {
        Self::Selector(component)
    }
}

impl<'a> From<KeyBindComponent<'a>> for Component<'a> {
    fn from(component: KeyBindComponent<'a>) -> Self {
        Self::KeyBind(component)
    }
}

impl<'a> From<BaseComponent<'a>> for Component<'a> {
    fn from(component: BaseComponent<'a>) -> Self {
        Self::Base(component)
    }
}