use iced::{
    Element, Theme,
    widget::{column, markdown, scrollable, space, text},
};

use crate::{Message, TEXT_SIZE, render_markdown};

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
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The update function may produce a Task for async background operations.")
                    .size(TEXT_SIZE),
                space().height(8.0),
                render_markdown(&self.md, theme),
                space().height(12.0),
                text("Task::perform takes an async function and a message constructor.")
                    .size(TEXT_SIZE),
                space().height(8.0),
                text(
                    "When the async work completes, the result is usually wrapped in the message."
                )
                .size(TEXT_SIZE),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
