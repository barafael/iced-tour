use iced::{
    Color, Element, Theme,
    widget::{column, container, markdown, row, scrollable, slider, space, text},
};

use crate::{FIRA_MONO, Message, SUBTITLE_COLOR, TEXT_SIZE, demo, render_markdown};

const MD_ROW_COL: &str = r#"
```rust
// Nested layouts
column![
    row![o(), b(), o(), b(), o()],
    row![b(), o(), b(), o(), b()],
    row![o(), b(), o(), b(), o()],
]
```
"#;

const PASTEL_ORANGE: Color = Color::from_rgb(1.0, 0.75, 0.5);
const PASTEL_BLUE: Color = Color::from_rgb(0.55, 0.75, 1.0);

const MD_CONTAINER: &str = r#"
```rust
// Wrap content for positioning and styling
container(content)
    .center_x(Fill)
    .center_y(Fill)
    .padding(20)
    .style(container::rounded_box)
```
"#;

const MD_SPACING: &str = r#"
```rust
column![a, b, c]
    .spacing(10)           // Gap between children
    .padding(20)           // Space around the column
    .align_x(Center)       // Horizontal alignment
```
"#;

pub struct LayoutSlide {
    md_row_col: Vec<markdown::Item>,
    md_container: Vec<markdown::Item>,
    md_spacing: Vec<markdown::Item>,
}

impl Default for LayoutSlide {
    fn default() -> Self {
        Self {
            md_row_col: markdown::parse(MD_ROW_COL).collect(),
            md_container: markdown::parse(MD_CONTAINER).collect(),
            md_spacing: markdown::parse(MD_SPACING).collect(),
        }
    }
}

impl LayoutSlide {
    pub fn view_row_col(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The building blocks of layout.").size(TEXT_SIZE),
                space().height(12.0),
                render_markdown(&self.md_row_col, theme),
                space().height(20.0),
                space().height(10.0),
                {
                    let cell = |color: Color| {
                        let s = 50.0;
                        container(space())
                            .width(s)
                            .height(s)
                            .style(move |_: &_| container::Style {
                                background: Some(color.into()),
                                ..Default::default()
                            })
                    };
                    let o = || cell(PASTEL_ORANGE);
                    let b = || cell(PASTEL_BLUE);
                    container(column![
                        row![o(), b(), o(), b(), o()],
                        row![b(), o(), b(), o(), b()],
                        row![o(), b(), o(), b(), o()],
                    ])
                    .style(container::rounded_box)
                    .clip(true)
                },
            ]
            .spacing(8.0),
        )
        .into()
    }

    pub fn view_container(&self, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.").size(TEXT_SIZE),
                space().height(12.0),
                render_markdown(&self.md_container, theme),
                space().height(20.0),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10.0),
                container(
                    container(text("Centered and styled"))
                        .padding(20.0)
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(8.0),
        )
        .into()
    }

    pub fn view_spacing(
        &self,
        theme: &Theme,
        demo: &demo::Demo,
        shift_held: bool,
    ) -> Element<'_, Message> {
        let sp = demo.spacing();
        let pd = demo.padding();

        let preview: Element<'_, Message> = container(
            column![text("A"), text("B"), text("C")]
                .spacing(sp)
                .align_x(iced::Alignment::Center),
        )
        .padding(pd)
        .style(container::rounded_box)
        .into();

        let preview = if shift_held {
            preview.explain(Color::from_rgb(0.4, 0.2, 0.8))
        } else {
            preview
        };

        let spacing_slider = row![
            text(format!(".spacing({:.0})", sp))
                .size(22)
                .font(FIRA_MONO),
            slider(0.0..=40.0, sp, |v| Message::Demo(
                demo::Message::SpacingChanged(v)
            ))
            .width(200.0),
        ]
        .spacing(12.0)
        .align_y(iced::Alignment::Center);

        let padding_slider = row![
            text(format!(".padding({:.0})", pd))
                .size(22)
                .font(FIRA_MONO),
            slider(0.0..=40.0, pd, |v| Message::Demo(
                demo::Message::PaddingChanged(v)
            ))
            .width(200.0),
        ]
        .spacing(12.0)
        .align_y(iced::Alignment::Center);

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(TEXT_SIZE),
                space().height(12.0),
                render_markdown(&self.md_spacing, theme),
                space().height(20.0),
                row![spacing_slider, padding_slider].spacing(20.0),
                space().height(12.0),
                preview,
                space().height(8.0),
                text("hint: press shift")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(8.0),
        )
        .into()
    }
}
