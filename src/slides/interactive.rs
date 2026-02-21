use iced::Element;

use crate::Message;

pub struct InteractiveSlide;

impl InteractiveSlide {
    pub fn view<'a>(&self, page_boop: &'a page_boop::PageBoop) -> Element<'a, Message> {
        page_boop.view().map(Message::PageBoop)
    }
}
