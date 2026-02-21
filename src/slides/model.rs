use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, ScaleCtx, TEXT_SIZE, render_markdown};

const MD_MODEL: &str = r#"
```rust
enum Mode {
    Title,
    DownloadTime,
    DownloadSize,
}

struct UrlAnalyzer {
    url: String,
    secure: bool,
    mode: Mode,
}
```
"#;

pub struct ModelSlide {
    md: Vec<markdown::Item>,
}

impl Default for ModelSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_MODEL).collect(),
        }
    }
}

impl ModelSlide {
    pub fn view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(12.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(12.0)),
                text("Notice: completely UI-agnostic.").size(ctx.sz(TEXT_SIZE)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
