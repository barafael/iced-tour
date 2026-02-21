use iced::{
    Color, Element, Theme,
    widget::button as iced_button,
    widget::{column, container, markdown, pick_list, row, scrollable, space, text},
};
use iced_anim::widget::button;

use crate::{Message, SUBTITLE_COLOR, ScaleCtx, TEXT_SIZE, render_markdown, theming};

const MD_VIEW: &str = r#"
```rust
fn view(&self) -> Element<Message> {
    column![
        text_input("URL", &self.url)
            .on_input(Message::UrlChanged),
        button("Get").on_press(Message::Action),
    ].into()
}
```
"#;

pub struct ViewSlide {
    md: Vec<markdown::Item>,
}

impl Default for ViewSlide {
    fn default() -> Self {
        Self {
            md: markdown::parse(MD_VIEW).collect(),
        }
    }
}

impl ViewSlide {
    pub fn view_view(&self, ctx: ScaleCtx, theme: &Theme) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(12.0)),
                render_markdown(&self.md, ctx, theme),
                space().height(ctx.sp(12.0)),
                text("Notice the method signature: &self (immutable borrow).")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(8.0)),
                text("The View can read state but never modify it.").size(ctx.sz(TEXT_SIZE)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }

    pub fn view_theming(&self, ctx: ScaleCtx, theming: &theming::Theming) -> Element<'_, Message> {
        let hover_color = theming.hover_color();

        let swatch =
            button(
                container(space().width(60).height(40)).style(move |_| container::Style {
                    background: Some(hover_color.into()),
                    ..Default::default()
                }),
            )
            .on_press(Message::Theming(theming::Message::OpenColorPicker));

        let picker = iced_aw::helpers::color_picker(
            theming.show_color_picker(),
            theming.hover_color(),
            swatch,
            Message::Theming(theming::Message::CancelColorPicker),
            |c| Message::Theming(theming::Message::SubmitColor(c)),
        );

        let demo_button = button(text("Hover me").size(ctx.sz(TEXT_SIZE)))
            .on_press(Message::Noop)
            .padding(ctx.sp(16.0))
            .style(move |theme: &Theme, status| match status {
                iced_button::Status::Hovered => iced_button::Style {
                    background: Some(hover_color.into()),
                    text_color: Color::WHITE,
                    ..iced_button::primary(theme, status)
                },
                _ => iced_button::primary(theme, status),
            });

        scrollable(
            column![
                text("Every widget has a .style() method. Pick a hover color for the button:")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(20.0)),
                row![
                    column![
                        text("Click to pick a color:")
                            .size(ctx.sz(TEXT_SIZE - 4))
                            .color(SUBTITLE_COLOR),
                        space().height(ctx.sp(8.0)),
                        picker,
                    ],
                    space().width(ctx.sp(40.0)),
                    column![
                        text("Styled button:")
                            .size(ctx.sz(TEXT_SIZE - 4))
                            .color(SUBTITLE_COLOR),
                        space().height(ctx.sp(8.0)),
                        demo_button,
                    ],
                ]
                .align_y(iced::Alignment::Start),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }

    pub fn view_theme_picker<'a>(
        &self,
        ctx: ScaleCtx,
        theming: &'a theming::Theming,
    ) -> Element<'a, Message> {
        scrollable(
            column![
                text("Iced ships with built-in themes you can switch at runtime.")
                    .size(ctx.sz(TEXT_SIZE)),
                space().height(ctx.sp(20.0)),
                row![
                    text("Theme:").size(ctx.sz(TEXT_SIZE)),
                    pick_list(Theme::ALL, Some(theming.theme()), |t| Message::Theming(
                        theming::Message::ThemeChanged(t)
                    ),),
                ]
                .spacing(ctx.sp(12.0))
                .align_y(iced::Alignment::Center),
                space().height(ctx.sp(24.0)),
                text("Sample widgets with the current theme:")
                    .size(ctx.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
                space().height(ctx.sp(12.0)),
                row![
                    button(text("Default").size(ctx.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(ctx.sp(12.0)),
                    button(text("Primary").size(ctx.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(ctx.sp(12.0))
                        .style(iced_button::primary),
                    button(text("Secondary").size(ctx.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(ctx.sp(12.0))
                        .style(iced_button::secondary),
                    button(text("Danger").size(ctx.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(ctx.sp(12.0))
                        .style(iced_button::danger),
                ]
                .spacing(ctx.sp(12.0)),
            ]
            .spacing(ctx.sp(8.0)),
        )
        .into()
    }
}
