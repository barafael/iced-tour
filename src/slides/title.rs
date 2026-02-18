use iced::{
    Element,
    widget::{column, container, row, space, svg, text},
};
use lucide_icons::iced::icon_keyboard;

use crate::{App, ICED_LOGO, Message, ORANGE, SUBTITLE_COLOR};

impl App {
    pub fn view_title_screen(&self) -> Element<'_, Message> {
        container(
            column![
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(self.sp(96.0))
                    .height(self.sp(96.0)),
                space().height(self.sp(30.0)),
                text("Discover Iced").size(self.sz(76)).color(ORANGE),
                space().height(self.sp(16.0)),
                text("A quick tour of Iced (made with Iced)")
                    .size(self.sz(30))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(50.0)),
                row![
                    icon_keyboard().size(self.sz(22)).color(SUBTITLE_COLOR),
                    text("Use arrow keys to navigate")
                        .size(self.sz(22))
                        .color(SUBTITLE_COLOR),
                ]
                .spacing(self.sp(8.0))
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
