use iced::{
    Color, Element,
    widget::{column, container, row, scrollable, slider, space, text},
};

use crate::{App, FIRA_MONO, Message, SUBTITLE_COLOR, TEXT_SIZE, demo};

pub const MD_ROW_COL: &str = r#"
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
                text("The building blocks of layout.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_row_col),
                space().height(self.sp(20.0)),
                space().height(self.sp(10.0)),
                {
                    let cell = |color: Color| {
                        let s = self.sp(50.0);
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
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_layout_container_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_container),
                space().height(self.sp(20.0)),
                text("Live example:")
                    .size(self.sz(TEXT_SIZE))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(10.0)),
                container(
                    container(text("Centered and styled"))
                        .padding(self.sp(20.0))
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_layout_spacing_screen(&self) -> Element<'_, Message> {
        let sp = self.demo.demo_spacing;
        let pd = self.demo.demo_padding;

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
                .size(self.sz(22))
                .font(FIRA_MONO),
            slider(0.0..=40.0, sp, |v| Message::Demo(
                demo::Message::SpacingChanged(v)
            ))
            .width(self.sp(200.0)),
        ]
        .spacing(self.sp(12.0))
        .align_y(iced::Alignment::Center);

        let padding_slider = row![
            text(format!(".padding({:.0})", pd))
                .size(self.sz(22))
                .font(FIRA_MONO),
            slider(0.0..=40.0, pd, |v| Message::Demo(
                demo::Message::PaddingChanged(v)
            ))
            .width(self.sp(200.0)),
        ]
        .spacing(self.sp(12.0))
        .align_y(iced::Alignment::Center);

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_spacing),
                space().height(self.sp(20.0)),
                row![spacing_slider, padding_slider].spacing(self.sp(20.0)),
                space().height(self.sp(12.0)),
                preview,
                space().height(self.sp(8.0)),
                text("hint: press shift")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
