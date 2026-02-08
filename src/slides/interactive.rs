use iced::Element;

use crate::{App, Message};

impl App {
    pub fn view_interactive_screen(&self) -> Element<'_, Message> {
        self.page_poker.view().map(Message::PagePoker)
    }
}
