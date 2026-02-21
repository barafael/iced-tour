use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, TEXT_SIZE, render_markdown};

const MD_MESSAGE: &str = r#"
```rust
enum Message {
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),
}
```
"#;

pub struct MessageSlide {
    md: Vec<markdown::Item>,
}

impl Default for MessageSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_MESSAGE).collect(),
        }
    }
}

impl MessageSlide {
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(TEXT_SIZE),
                space().height(8.0),
                render_markdown(&self.md, theme),
                space().height(12.0),
                text("Messages are produced by the view.").size(TEXT_SIZE)
            ]
            .spacing(8.0),
        )
        .into()
    }
}
