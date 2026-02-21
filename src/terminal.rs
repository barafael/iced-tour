use iced::Font;

pub struct Terminal {
    pub term: iced_term::Terminal,
}

#[derive(Debug, Clone)]
pub enum Message {
    TermEvent(iced_term::Event),
}

pub enum Action {
    None,
}

impl Terminal {
    pub fn new(mono_font: Font) -> Self {
        Self {
            term: shell_backend(mono_font),
        }
    }

    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::TermEvent(iced_term::Event::BackendCall(_, cmd)) => {
                self.term.handle(iced_term::Command::ProxyToBackend(cmd));
            }
        }
        Action::None
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        self.term.subscription().map(Message::TermEvent)
    }
}

fn shell_backend(mono_font: Font) -> iced_term::Terminal {
    #[cfg(not(windows))]
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    #[cfg(windows)]
    let shell = "cmd.exe".to_string();
    let settings = iced_term::settings::Settings {
        font: iced_term::settings::FontSettings {
            size: 14.0,
            font_type: mono_font,
            ..Default::default()
        },
        backend: iced_term::settings::BackendSettings {
            program: shell,
            ..Default::default()
        },
        ..Default::default()
    };
    iced_term::Terminal::new(0, settings).expect("failed to create terminal")
}
