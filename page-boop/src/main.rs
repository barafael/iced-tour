use iced::{Element, Font, Task};
use page_boop::{FIRA_MONO_BYTES, PageBoop};

const FIRA_MONO: Font = Font::with_name("Fira Mono");

struct App {
    boop: PageBoop,
}

#[derive(Debug, Clone)]
enum Message {
    Boop(page_boop::Message),
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
            boop: PageBoop::new(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Boop(msg) => self.boop.update(msg).map(Message::Boop),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::container(self.boop.view().map(Message::Boop))
            .padding(30)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }
}
