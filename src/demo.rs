pub struct Demo {
    pub button_clicks: u32,
    pub input_changes: u32,
    pub input_submits: u32,
    pub demo_input: String,
    pub demo_spacing: f32,
    pub demo_padding: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked,
    InputChanged(String),
    InputSubmitted,
    SpacingChanged(f32),
    PaddingChanged(f32),
}

pub enum Action {
    None,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            button_clicks: 0,
            input_changes: 0,
            input_submits: 0,
            demo_input: String::new(),
            demo_spacing: 10.0,
            demo_padding: 10.0,
        }
    }
}

impl Demo {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ButtonClicked => {
                self.button_clicks += 1;
            }
            Message::InputChanged(value) => {
                self.demo_input = value;
                self.input_changes += 1;
            }
            Message::InputSubmitted => {
                self.input_submits += 1;
            }
            Message::SpacingChanged(val) => {
                self.demo_spacing = val;
            }
            Message::PaddingChanged(val) => {
                self.demo_padding = val;
            }
        }
        Action::None
    }
}
