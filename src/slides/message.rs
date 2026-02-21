use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, ScaleCtx, TEXT_SIZE, render_markdown};

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
    pub fn view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(12.0)),
                text("Messages are produced by the view.").size(ctx.sz(TEXT_SIZE))
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
