use iced::{Element, Length, widget::container};
use iced_term::TerminalView;

use crate::{App, Message};

impl App {
    pub fn view_community_widgets_screen(&self) -> Element<'_, Message> {
        container(TerminalView::show(&self.term).map(Message::TermEvent))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(self.sp(4.0))
            .style(|_| container::Style {
                background: Some(iced::Color::BLACK.into()),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .padding(self.sp(10.0))
            .height(Length::Fill)
            .into()
    }
}
