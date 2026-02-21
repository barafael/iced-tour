use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, SUBTITLE_COLOR, TEXT_SIZE, render_markdown};

const MD_CONSTRUCTORS: &str = r#"
```rust
// these two are equivalent:
.on_input(Message::UrlChanged)
.on_input(|s| Message::UrlChanged(s))
```
"#;

const MD_WIDGET_MESSAGES: &str = r#"
```rust
// Each widget sends its state into your Message:
text_input(..).on_input(f)    // f: impl Fn(String) -> Message
toggler(..).on_toggle(f)      // f: impl Fn(bool) -> Message
button(..).on_press(message)  // f: impl Fn(()) -> Message
```
"#;

pub struct ConstructorsSlide {
    md_constructors: Vec<markdown::Item>,
    md_widget_messages: Vec<markdown::Item>,
}

impl Default for ConstructorsSlide {
    fn default() -> Self {
        Self {
            md_constructors: markdown::parse(MD_CONSTRUCTORS).collect(),
            md_widget_messages: markdown::parse(MD_WIDGET_MESSAGES).collect(),
        }
    }
}

impl ConstructorsSlide {
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Enum variants with data are enum constructors.").size(TEXT_SIZE),
                space().height(8.0),
                render_markdown(&self.md_constructors, theme),
                space().height(16.0),
                text("Widgets pass their state into these constructors:")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
                space().height(8.0),
                render_markdown(&self.md_widget_messages, theme),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
