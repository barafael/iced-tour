use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub const MD_MODEL: &str = r#"
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

impl App {
    pub fn view_model_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_model),
                space().height(12),
                text("Notice: completely UI-agnostic.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
