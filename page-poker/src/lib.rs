use iced::{
    Color, Element, Font, Length, Task,
    widget::{
        button, checkbox, column, container, pick_list, row, scrollable, space, text, text_input,
    },
};
use serde::Serialize;
use strum::{Display, EnumIter, IntoEnumIterator};

/// Mono font embedded from the shared fonts directory.
pub const FIRA_MONO_BYTES: &[u8] = include_bytes!("../../fonts/FiraMono-Regular.ttf");

const FIRA_MONO: Font = Font::with_name("Fira Mono");

/// Injectable style configuration so the host app can match its own theme.
pub struct StyleConfig {
    pub mono_font: Font,
    pub subtitle_color: Color,
    pub text_size: u32,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            mono_font: FIRA_MONO,
            subtitle_color: Color::from_rgb(0.45, 0.45, 0.45),
            text_size: 24,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize)]
pub enum Mode {
    #[default]
    Title,

    #[strum(serialize = "Download Time")]
    DownloadTime,

    #[strum(serialize = "Download Size")]
    DownloadSize,
}

#[derive(Default, Serialize)]
pub struct UrlAnalyzer {
    url: String,
    secure: bool,
    mode: Mode,

    #[serde(skip_serializing_if = "String::is_empty")]
    result: String,

    #[serde(skip)]
    loading: bool,
}

/// Self-contained "Page Poker" component state.
pub struct PagePoker {
    model: UrlAnalyzer,
    message_log: Vec<String>,
    style: StyleConfig,
}

impl PagePoker {
    /// Create a new PagePoker with default styling.
    pub fn new() -> Self {
        Self {
            model: UrlAnalyzer::default(),
            message_log: Vec::new(),
            style: StyleConfig::default(),
        }
    }

    /// Create a new PagePoker with custom styling.
    pub fn with_style(style: StyleConfig) -> Self {
        Self {
            model: UrlAnalyzer::default(),
            message_log: Vec::new(),
            style,
        }
    }

    fn log_message(&mut self, msg: String) {
        self.message_log.push(msg);
    }

    /// Process a message and return a task.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlChanged(url) => {
                self.log_message(format!("UrlChanged({:?})", url));
                self.model.url = url;
                Task::none()
            }
            Message::SecureChanged(secure) => {
                self.log_message(format!("SecureChanged({})", secure));
                self.model.secure = secure;
                Task::none()
            }
            Message::ModeChanged(mode) => {
                self.log_message(format!("ModeChanged({})", mode));
                self.model.mode = mode;
                Task::none()
            }
            Message::Action => {
                self.log_message("Action".to_string());
                if self.model.url.is_empty() {
                    self.model.result = "Please enter a URL".to_string();
                    Task::none()
                } else {
                    self.model.loading = true;
                    self.model.result.clear();
                    let url = self.model.url.clone();
                    let secure = self.model.secure;
                    let mode = self.model.mode;
                    Task::perform(fetch_url(url, secure, mode), Message::Result)
                }
            }
            Message::Result(result) => {
                self.log_message(format!("Result({:?})", result));
                self.model.loading = false;
                self.model.result = result;
                Task::none()
            }
        }
    }

    /// Render the Page Poker UI.
    pub fn view(&self) -> Element<'_, Message> {
        let mono = self.style.mono_font;
        let subtitle = self.style.subtitle_color;
        let ts = self.style.text_size;

        let mode_options: Vec<Mode> = Mode::iter().collect();

        let get_button = if self.model.loading {
            button("Loading...")
        } else {
            button("Get").on_press(Message::Action)
        };

        let result_text = if self.model.loading {
            "Fetching...".to_string()
        } else if !self.model.result.is_empty() {
            self.model.result.clone()
        } else {
            "Enter a URL and click Get".to_string()
        };

        // RON state visualization
        let ron_config = ron::ser::PrettyConfig::default();
        let state_ron = ron::ser::to_string_pretty(&self.model, ron_config)
            .unwrap_or_else(|e| format!("Error: {e}"));

        // Message log
        let message_log_content: Element<'_, Message> = if self.message_log.is_empty() {
            text("Messages will appear here...")
                .size(20)
                .color(subtitle)
                .into()
        } else {
            column(
                self.message_log
                    .iter()
                    .map(|msg| {
                        row![text(msg).size(20).font(mono), space().width(Length::Fill)].into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(4)
            .into()
        };

        column![
            // Input row
            row![
                text_input("Enter URL (e.g. example.com)", &self.model.url)
                    .on_input(Message::UrlChanged)
                    .on_submit(Message::Action),
                checkbox(self.model.secure)
                    .label("HTTPS")
                    .on_toggle(Message::SecureChanged),
                pick_list(mode_options, Some(self.model.mode), Message::ModeChanged),
                get_button,
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center),
            space().height(24),
            // Result
            text(result_text).size(ts),
            space().height(36),
            // State and messages side by side
            row![
                column![
                    text("Current State").size(22).font(mono).color(subtitle),
                    space().height(8),
                    container(text(state_ron).size(20).font(mono))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(Length::FillPortion(1)),
                column![
                    text("Recent Messages").size(22).font(mono).color(subtitle),
                    space().height(8),
                    container(scrollable(message_log_content).height(150))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(Length::FillPortion(1)),
            ]
            .spacing(20),
        ]
        .width(iced::Fill)
        .into()
    }
}

impl Default for PagePoker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),
}

/// Fetch a URL and return a result string based on the chosen mode.
pub async fn fetch_url(url: String, secure: bool, mode: Mode) -> String {
    let protocol = if secure { "https" } else { "http" };
    let full_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("{protocol}://{url}")
    };

    let start = std::time::Instant::now();

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = match reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
    {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let response = match client.get(&full_url).send().await {
        Ok(resp) => resp,
        Err(e) => return format!("Error: {}", e),
    };

    let elapsed = start.elapsed();

    match mode {
        Mode::Title => {
            let body = match response.text().await {
                Ok(text) => text,
                Err(e) => return format!("Error reading body: {}", e),
            };

            if let Some(start_idx) = body.find("<title>")
                && let Some(end_idx) = body.find("</title>")
            {
                let title = &body[start_idx + 7..end_idx];
                return format!("Title: {}", title.trim());
            }
            "No <title> found".to_string()
        }
        Mode::DownloadTime => match response.bytes().await {
            Ok(_) => format!("Download time: {elapsed:.2?}"),
            Err(error) => format!("Error: {error}"),
        },
        Mode::DownloadSize => match response.bytes().await {
            Ok(bytes) => format!(
                "Size: {}",
                humansize::format_size(bytes.len(), humansize::DECIMAL)
            ),
            Err(error) => format!("Error: {error}"),
        },
    }
}
