use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, ScaleCtx, TEXT_SIZE, render_markdown};

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
    pub fn view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(12.0)),
                text("Notice the method signature! (&mut)").size(ctx.sz(TEXT_SIZE)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
