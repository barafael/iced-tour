use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text, text_input},
};

use crate::{Message, ScaleCtx, TEXT_SIZE, demo, render_markdown};

const MD_TEXT_INPUT: &str = r#"
```rust
text_input("Enter URL (e.g. example.com)", &self.model.url)
    .on_input(Message::UrlChanged)
    .on_submit(Message::Action)
```
"#;

pub struct TextInputSlide {
    md: Vec<markdown::Item>,
}

impl Default for TextInputSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_TEXT_INPUT).collect(),
        }
    }
}

impl TextInputSlide {
    pub fn view<'a>(
        &'a self,
        ctx: ScaleCtx,
        theme: &Theme,
        demo: &demo::Demo,
    ) -> Element<'a, Message> {
        scrollable(
            column![
                text("The Text Input widget produces messages as the user types.")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(20.0)),
                text_input("Enter URL (e.g. example.com)", demo.input_text())
                    .on_input(|s| Message::Demo(demo::Message::InputChanged(s)))
                    .on_submit(Message::Demo(demo::Message::InputSubmitted)),
                space().height(ctx.sp(12.0)),
                text!("Input Changed messages: {}", demo.input_changes()).size(ctx.sz(TEXT_SIZE)),
                text!("Input Submitted messages: {}", demo.input_submits()).size(ctx.sz(TEXT_SIZE)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
