use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};
use iced_anim::widget::button;

use crate::{Message, SUBTITLE_COLOR, TEXT_SIZE, chaos, render_markdown};

const MD_SUBSCRIPTIONS: &str = r#"
```rust
fn subscription(&self) -> Subscription<Message> {
    event::listen_with(|event, _, _| match event {
        Event::Keyboard(KeyPressed {
            key: Key::Named(Named::ArrowRight), ..
        }) => Some(Message::NextScreen),
        ...
    })
}
```
"#;

pub struct SubscriptionsSlide {
    md: Vec<markdown::Item>,
}

impl Default for SubscriptionsSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_SUBSCRIPTIONS).collect(),
        }
    }
}

impl SubscriptionsSlide {
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Subscriptions feed external (asynchronous) events into your app.")
                    .size(TEXT_SIZE),
                space().height(12.0),
                render_markdown(&self.md, theme),
                space().height(16.0),
                space().height(8.0),
                text("This slideshow listens to keyboard events.")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                space().height(16.0),
                text("Other common uses: timers, window events, WebSocket messages.")
                    .size(TEXT_SIZE),
                space().height(12.0),
                button("🚨 Panic!").on_press(Message::Chaos(chaos::Message::PanicChaos)),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
