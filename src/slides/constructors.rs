use iced::{
    Element,
    widget::{column, scrollable, space, text},
};

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_CONSTRUCTORS: &str = r#"
```rust
// these two are equivalent:
.on_input(Message::UrlChanged)
.on_input(|s| Message::UrlChanged(s))
```
"#;

pub const MD_WIDGET_MESSAGES: &str = r#"
```rust
// Each widget sends its state into your Message:
text_input(..).on_input(f)    // f: impl Fn(String) -> Message
toggler(..).on_toggle(f)      // f: impl Fn(bool) -> Message
button(..).on_press(message)  // message: Message (no fn, just a value)
```
"#;

impl App {
    pub fn view_constructors_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Enum variants with data are enum constructors.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_constructors),
                space().height(16),
                text("Widgets pass their state into these constructors:")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
                space().height(8),
                self.md_container(&self.md_widget_messages),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
