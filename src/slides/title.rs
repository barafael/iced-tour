use iced::{
    Element,
    widget::{column, container, row, space, svg, text},
};
use lucide_icons::iced::icon_keyboard;

use crate::{ICED_LOGO, Message, ORANGE, SUBTITLE_COLOR};

pub struct TitleSlide;

impl TitleSlide {
    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(96.0)
                    .height(96.0),
                space().height(30.0),
                text("Discover Iced").size(76).color(ORANGE),
                space().height(16.0),
                text("A quick tour of Iced (made with Iced)")
                    .size(30)
                    .color(SUBTITLE_COLOR),
                space().height(50.0),
                row![
                    icon_keyboard().size(22).color(SUBTITLE_COLOR),
                    text("Use arrow keys to navigate")
                        .size(22)
                        .color(SUBTITLE_COLOR),
                ]
                .spacing(8.0)
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
