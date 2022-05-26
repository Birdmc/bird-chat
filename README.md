# cubic-chat
Cubic-chat is implementation of minecraft chat components on rust with serde support.

# Installing
https://crates.io/crates/cubic-chat

# Usage
```rs
let mut component = TextComponent::new("hi".into());
component.base.bold = Some(true);
component.base.color = Some(DefaultColor::Red.into());
component.base.extra = vec![
  {
    let mut component = TextComponent::new("bye".into());
    component.base.color = Some(DefaultColor::White.into());
    component.base.bold = Some(false);
    component.into()
  }
];
```
