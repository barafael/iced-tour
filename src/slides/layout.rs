use iced::{
    Color, Element,
    widget::{button, column, container, row, scrollable, slider, space, text, text_input},
};

use crate::{App, FIRA_MONO, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_ROW_COL: &str = r#"
```rust
// Nested layouts
column![
    row![text("Name"), text_input("Type here...", &self.input)],
    row![button("Cancel"), button("Submit")],
]
```
"#;

pub const MD_CONTAINER: &str = r#"
```rust
// Wrap content for positioning and styling
container(content)
    .center_x(Fill)
    .center_y(Fill)
    .padding(20)
    .style(container::rounded_box)
```
"#;

pub const MD_SPACING: &str = r#"
```rust
column![a, b, c]
    .spacing(10)           // Gap between children
    .padding(20)           // Space around the column
    .align_x(Center)       // Horizontal alignment
```
"#;

impl App {
    pub fn view_layout_row_col_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The building blocks of layout.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_row_col),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    column![
                        row![
                            text("Name"),
                            text_input("Type here...", &self.demo_input)
                                .on_input(Message::DemoInputChanged)
                        ]
                        .spacing(10)
                        .align_y(iced::Alignment::Center),
                        row![
                            button("Cancel").on_press(Message::Noop),
                            button("Submit").on_press(Message::Noop),
                        ]
                        .spacing(10),
                    ]
                    .spacing(10)
                )
                .padding(15)
                .style(container::rounded_box),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    pub fn view_layout_container_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_container),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    container(text("Centered and styled"))
                        .padding(20)
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    pub fn view_layout_spacing_screen(&self) -> Element<'_, Message> {
        let sp = self.demo_spacing;
        let pd = self.demo_padding;

        // Interactive preview driven by sliders
        let preview: Element<'_, Message> = container(
            column![text("A"), text("B"), text("C")]
                .spacing(sp)
                .align_x(iced::Alignment::Center),
        )
        .padding(pd)
        .style(container::rounded_box)
        .into();

        let preview = if self.shift_held {
            preview.explain(Color::from_rgb(0.4, 0.2, 0.8))
        } else {
            preview
        };

        let spacing_slider = row![
            text(format!(".spacing({:.0})", sp))
                .size(16)
                .font(FIRA_MONO),
            slider(0.0..=40.0, sp, Message::DemoSpacingChanged).width(200),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center);

        let padding_slider = row![
            text(format!(".padding({:.0})", pd))
                .size(16)
                .font(FIRA_MONO),
            slider(0.0..=40.0, pd, Message::DemoPaddingChanged).width(200),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center);

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_spacing),
                space().height(20),
                row![spacing_slider, padding_slider].spacing(20),
                space().height(12),
                preview,
                space().height(8),
                text("hint: press shift")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
