use iced::{Element, Length, widget::container};
use iced_term::TerminalView;

use crate::{Message, ScaleCtx, terminal};

pub struct CommunityWidgetsSlide;

impl CommunityWidgetsSlide {
    pub fn view<'a>(
        &self,
        ctx: ScaleCtx,
        terminal: &'a terminal::Terminal,
    ) -> Element<'a, Message> {
        container(
            TerminalView::show(terminal.term())
                .map(|e| Message::Terminal(terminal::Message::TermEvent(e))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(ctx.sp(4.0))
        .style(|_| container::Style {
            background: Some(iced::Color::BLACK.into()),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding(ctx.sp(10.0))
        .height(Length::Fill)
        .into()
    }
}
