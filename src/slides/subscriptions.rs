use iced::{
    Element,
    widget::{column, scrollable, space, text},
};
use iced_anim::widget::button;

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE, chaos};

pub const MD_SUBSCRIPTIONS: &str = r#"
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

impl App {
    pub fn view_subscriptions_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Subscriptions feed external (asynchronous) events into your app.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_subscriptions),
                space().height(self.sp(16.0)),
                space().height(self.sp(8.0)),
                text("This slideshow listens to keyboard events.")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(16.0)),
                text("Other common uses: timers, window events, WebSocket messages.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                button("🚨 Panic!").on_press(Message::Chaos(chaos::Message::PanicChaos)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
