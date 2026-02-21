use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, svg},
};

use crate::{ELM_CIRCLE_OF_LIFE, Message, ScaleCtx, render_markdown};

const MD_INTRO: &str = r#"
The **Elm Architecture** is a pattern for structuring interactive applications.

It separates concerns into four distinct parts:

1. **Model** — the application state
2. **Message** — events from user input or the system
3. **Update** — a function that applies messages to the model
4. **View** — transforms state into UI with event handlers
"#;

pub struct IntroSlide {
    md: Vec<markdown::Item>,
}

impl Default for IntroSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_INTRO).collect(),
        }
    }
}

impl IntroSlide {
    pub fn view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(30.0)),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(ctx.sp(220.0)),
                space().height(ctx.sp(30.0)),
            ]
            .align_x(iced::Alignment::Center),
        )
        .into()
    }
}
