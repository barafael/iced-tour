use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, ScaleCtx, TEXT_SIZE, render_markdown};

const MD_TASKS: &str = r#"
```rust
fn update(&mut self, message: Message) -> Task<Message> {
    ...
    Message::Action => {
        return Task::perform(
            fetch_url(self.url.clone(), self.secure, self.mode),
            Message::Result,
        );
    }
    ...
}
```
"#;

pub struct TasksSlide {
    md: Vec<markdown::Item>,
}

impl Default for TasksSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_TASKS).collect(),
        }
    }
}

impl TasksSlide {
    pub fn view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The update function may produce a Task for async background operations.")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(12.0)),
                text("Task::perform takes an async function and a message constructor.")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                text(
                    "When the async work completes, the result is usually wrapped in the message."
                )
                .size(ctx.sz(TEXT_SIZE)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
