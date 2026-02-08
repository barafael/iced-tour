use iced::{Element, Font, Task};
use page_poker::{FIRA_MONO_BYTES, PagePoker};

const FIRA_MONO: Font = Font::with_name("Fira Mono");

struct App {
    poker: PagePoker,
}

#[derive(Debug, Clone)]
enum Message {
    Poker(page_poker::Message),
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Page Poker")
        .font(FIRA_MONO_BYTES)
        .default_font(FIRA_MONO)
        .run()
}

impl Default for App {
    fn default() -> Self {
        Self {
            poker: PagePoker::new(),
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
