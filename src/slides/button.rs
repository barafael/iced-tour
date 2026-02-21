use iced::{
    Element, Theme,
    widget::{column, markdown, row, scrollable, space, text},
};
use iced_anim::widget::button;

use crate::{Message, ScaleCtx, TEXT_SIZE, demo, render_markdown};

const MD_BUTTON: &str = r#"
```rust
button("Get").on_press(Message::Action)
```
"#;

pub struct ButtonSlide {
    md: Vec<markdown::Item>,
}

impl Default for ButtonSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_BUTTON).collect(),
        }
    }
}

impl ButtonSlide {
    pub fn view<'a>(
        &'a self,
        ctx: ScaleCtx,
        theme: &Theme,
        demo: &demo::Demo,
    ) -> Element<'a, Message> {
        let click_text = if demo.button_clicks() == 0 {
            String::from("Click the button!")
        } else {
            format!(
                "Clicked {} time{}",
                demo.button_clicks(),
                if demo.button_clicks() == 1 { "" } else { "s" }
            )
        };

        scrollable(
            column![
                text("The Button widget produces messages when clicked.").size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(20.0)),
                row![
                    button("Get").on_press(Message::Demo(demo::Message::ButtonClicked)),
                    text(click_text).size(ctx.sz(TEXT_SIZE)),
                ]
                .spacing(ctx.sp(15.0))
                .align_y(iced::Alignment::Center),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
