use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, TEXT_SIZE, render_markdown};

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
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(TEXT_SIZE),
                space().height(12.0),
                render_markdown(&self.md, theme),
                space().height(12.0),
                text("Notice: completely UI-agnostic.").size(TEXT_SIZE),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
