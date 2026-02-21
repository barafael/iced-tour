use iced::{Color, Theme};

pub struct Theming {
    hover_color: Color,
    show_color_picker: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenColorPicker,
    SubmitColor(Color),
    CancelColorPicker,
    ThemeChanged(Theme),
}

pub enum Action {
    None,
    ThemeChanged,
}

impl Default for Theming {
    fn default() -> Self {
        Self {
            hover_color: Color::from_rgb(0.3, 0.7, 1.0),
            show_color_picker: false,
            theme: Theme::GruvboxLight,
        }
    }
}

impl Theming {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::OpenColorPicker => {
                self.show_color_picker = true;
                Action::None
            }
            Message::SubmitColor(color) => {
                self.hover_color = color;
                self.show_color_picker = false;
                Action::None
            }
            Message::CancelColorPicker => {
                self.show_color_picker = false;
                Action::None
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Action::ThemeChanged
            }
        }
    }

    pub fn hover_color(&self) -> Color {
        self.hover_color
    }

    pub fn show_color_picker(&self) -> bool {
        self.show_color_picker
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}
