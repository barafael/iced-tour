use iced::{
    Element,
    widget::{column, scrollable, space, text},
};

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_THEME: &str = r#"
```rust
button("Click me")
    .style(|theme, status| {
        match status {
            button::Status::Active => button::Style {
                background: Some(Color::from_rgb(0.2, 0.6, 1.0).into()),
                text_color: Color::WHITE,
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Color::from_rgb(0.3, 0.7, 1.0).into()),
                text_color: Color::WHITE,
                ..Default::default()
            },
            _ => button::primary(theme, status),
        }
    })
```
"#;

pub const MD_VIEW: &str = r#"
```rust
fn view(&self) -> Element<Message> {
    column![
        text_input("URL", &self.url)
            .on_input(Message::UrlChanged),
        button("Get").on_press(Message::Action),
    ].into()
}
```
"#;

impl App {
    pub fn view_theming_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Every widget has a .style() method that takes a closure.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_theme),
                space().height(12),
                text("The closure receives the current Theme and widget Status (Active, Hovered, Pressed, …).")
                    .size(TEXT_SIZE),
                space().height(8),
                text("Return a Style struct — background, text_color, border, shadow, …")
                    .size(TEXT_SIZE),
                space().height(16),
                text("Iced also ships with built-in themes (GruvboxLight, Dracula, Nord, …) that you can switch at runtime.")
                    .size(TEXT_SIZE - 2)
                    .color(SUBTITLE_COLOR),
                space().height(4),
                text("hint: hold Ctrl to try it yourself")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    pub fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_view),
                space().height(12),
                text("Notice the method signature: &self (immutable borrow).").size(TEXT_SIZE),
                space().height(8),
                text("The View can read state but never modify it.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
