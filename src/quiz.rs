#[derive(Default)]
pub struct Quiz {
    pub answer: Option<u8>,
    pub http_answer: Option<u8>,
    pub button_answer: Option<u8>,
    pub validation_answer: Option<u8>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Answer(u8),
    HttpAnswer(u8),
    ButtonAnswer(u8),
    ValidationAnswer(u8),
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
            Message::ValidationAnswer(a) => self.validation_answer = Some(a),
        }
        Action::None
    }
}
