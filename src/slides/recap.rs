use iced::{
    Element,
    widget::{column, container, space, svg, text},
};

use crate::{ELM_CIRCLE_OF_LIFE, Message, ORANGE, SUBTITLE_COLOR, TEXT_SIZE};

pub struct RecapSlide;

impl RecapSlide {
    pub fn view_takeaways(&self) -> Element<'_, Message> {
        let bullet = |s: &str| text(format!("  •  {s}")).size(TEXT_SIZE);
        let detail = |s: &str| {
            text(format!("       {s}"))
                .size(TEXT_SIZE - 4)
                .color(SUBTITLE_COLOR)
        };

        container(
            column![
                bullet("Model holds all state — View reads it, Update writes it"),
                detail("No hidden state, no side effects in View."),
                space().height(12.0),
                bullet("Messages are just data — they describe what happened"),
                detail("Not commands. The Update function decides what to do."),
                space().height(12.0),
                bullet("Side effects (usually) live in Tasks, not in Update directly"),
                detail("HTTP, file I/O, timers → Task::perform returns a Message."),
                space().height(12.0),
                bullet("Layout = nesting row! and column! with spacing/padding/alignment/..."),
                detail("Container for positioning, scrollable for overflow."),
                space().height(12.0),
                bullet("To feed messages from the external world, use subscriptions"),
                detail("Subscriptions just generate Messages."),
            ]
            .spacing(4.0)
            .padding(20.0),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    pub fn view_recap(&self) -> Element<'_, Message> {
        container(
            column![
                text("The Elm Architecture").size(54).color(ORANGE),
                space().height(30.0),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(280.0),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}
