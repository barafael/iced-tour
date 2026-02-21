pub struct Demo {
    button_clicks: u32,
    input_changes: u32,
    input_submits: u32,
    input_text: String,
    spacing: f32,
    padding: f32,
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
            input_text: String::new(),
            spacing: 10.0,
            padding: 10.0,
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
                self.input_text = value;
                self.input_changes += 1;
            }
            Message::InputSubmitted => {
                self.input_submits += 1;
            }
            Message::SpacingChanged(val) => {
                self.spacing = val;
            }
            Message::PaddingChanged(val) => {
                self.padding = val;
            }
        }
        Action::None
    }

    pub fn button_clicks(&self) -> u32 {
        self.button_clicks
    }

    pub fn input_changes(&self) -> u32 {
        self.input_changes
    }

    pub fn input_submits(&self) -> u32 {
        self.input_submits
    }

    pub fn input_text(&self) -> &str {
        &self.input_text
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }
}
