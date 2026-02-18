use iced::{Element, Font, Task};
use page_boop::{FIRA_MONO_BYTES, PageBoop};

const FIRA_MONO: Font = Font::with_name("Fira Mono");

struct App {
    poker: PageBoop,
}

#[derive(Debug, Clone)]
enum Message {
    Poker(page_boop::Message),
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Page Boop")
        .font(FIRA_MONO_BYTES)
        .default_font(FIRA_MONO)
        .run()
}

impl Default for App {
    fn default() -> Self {
        Self {
            poker: PageBoop::new(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Poker(msg) => self.poker.update(msg).map(Message::Poker),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::container(self.poker.view().map(Message::Poker))
            .padding(30)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }
}
