use iced::{
    Element,
    widget::{column, container, row, space, svg, text},
};
use lucide_icons::iced::icon_keyboard;

use crate::{ICED_LOGO, Message, ORANGE, SUBTITLE_COLOR, ScaleCtx};

pub struct TitleSlide;

impl TitleSlide {
    pub fn view(&self, ctx: ScaleCtx) -> Element<'_, Message> {
        container(
            column![
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(ctx.sp(96.0))
                    .height(ctx.sp(96.0)),
                space().height(ctx.sp(30.0)),
                text("Discover Iced").size(ctx.sz(76)).color(ORANGE),
                space().height(ctx.sp(16.0)),
                text("A quick tour of Iced (made with Iced)")
                    .size(ctx.sz(30))
                    .color(SUBTITLE_COLOR),
                space().height(ctx.sp(50.0)),
                row![
                    icon_keyboard().size(ctx.sz(22)).color(SUBTITLE_COLOR),
                    text("Use arrow keys to navigate")
                        .size(ctx.sz(22))
                        .color(SUBTITLE_COLOR),
                ]
                .spacing(ctx.sp(8.0))
                .align_y(iced::Alignment::Center),
            ]
            .width(iced::Fill)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}
