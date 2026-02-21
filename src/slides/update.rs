use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, TEXT_SIZE, render_markdown};

const MD_UPDATE: &str = r#"
```rust
fn update(&mut self, message: Message) {
    match message {
        Message::UrlChanged(url) => self.url = url,
        Message::SecureChanged(secure) => self.secure = secure,
        Message::ModeChanged(mode) => self.mode = mode,
        Message::Action => todo!("Start fetching URL"),
        Message::Result(result) => self.result = result,
    }
}
```
"#;

pub struct UpdateSlide {
    md: Vec<markdown::Item>,
}

impl Default for UpdateSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_UPDATE).collect(),
        }
    }
}

impl UpdateSlide {
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(TEXT_SIZE),
                space().height(8.0),
                render_markdown(&self.md, theme),
                space().height(12.0),
                text("Notice the method signature! (&mut)").size(TEXT_SIZE),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
