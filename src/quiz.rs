#[derive(Default)]
pub struct Quiz {
    answer: Option<u8>,
    http_answer: Option<u8>,
    button_answer: Option<u8>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Answer(u8),
    HttpAnswer(u8),
    ButtonAnswer(u8),
}

pub enum Action {
    None,
}

impl Quiz {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Answer(a) => self.answer = Some(a),
            Message::HttpAnswer(a) => self.http_answer = Some(a),
            Message::ButtonAnswer(a) => self.button_answer = Some(a),
        }
        Action::None
    }

    pub fn answer(&self) -> Option<u8> {
        self.answer
    }

    pub fn http_answer(&self) -> Option<u8> {
        self.http_answer
    }

    pub fn button_answer(&self) -> Option<u8> {
        self.button_answer
    }
}
