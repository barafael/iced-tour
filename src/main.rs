use iced::{
    Color, Element, Event, Font, Length, Padding, Subscription, Task, Theme, event, keyboard,
    widget::{
        button, canvas, checkbox, column, container, markdown, pick_list, row, scrollable, space,
        stack, svg, text, text_input, themer,
    },
};
use iced_anim::{Animate, Animated, Animation, Motion};
use lucide_icons::LUCIDE_FONT_BYTES;
use serde::Serialize;
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

mod chaos;
mod screen;
mod theme;
use theme::AppTheme;

use crate::screen::Screen;

#[derive(Debug, Clone, PartialEq, Animate)]
pub struct SlideOffset {
    left: f32,
    right: f32,
}

impl SlideOffset {
    fn settled() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }

    fn entering_forward() -> Self {
        Self {
            left: 40.0,
            right: 0.0,
        }
    }

    fn entering_backward() -> Self {
        Self {
            left: 0.0,
            right: 40.0,
        }
    }
}

const BITTER: Font = Font::with_name("Bitter");
const FIRA_MONO: Font = Font::with_name("Fira Mono");

const ICED_LOGO: &[u8] = include_bytes!("../assets/iced-logo.svg");

const TEXT_SIZE: u32 = 22;
const CODE_SIZE: u32 = 20;
const ORANGE: Color = Color::from_rgb(1.0, 0.4, 0.0);
const SUBTITLE_COLOR: Color = Color::from_rgb(0.45, 0.45, 0.45);

const ELM_CIRCLE_OF_LIFE: &[u8] = include_bytes!("../assets/elm.svg");

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

pub struct App {
    screen: Screen,
    slide_offset: Animated<SlideOffset>,
    model: UrlAnalyzer,
    theme: Theme,
    ctrl_held: bool,
    shift_held: bool,
    chaos_circles: Vec<chaos::ChaosCircle>,
    button_clicks: u32,
    input_changes: u32,
    input_submits: u32,
    demo_input: String,
    quiz_answer: Option<u8>,
    quiz_http_answer: Option<u8>,
    quiz_button_answer: Option<u8>,
    quiz_validation_answer: Option<u8>,
    message_log: Vec<String>,

    // Cached markdown content for each screen
    md_intro: Vec<markdown::Item>,
    md_model: Vec<markdown::Item>,
    md_view: Vec<markdown::Item>,
    md_row_col: Vec<markdown::Item>,
    md_container: Vec<markdown::Item>,
    md_spacing: Vec<markdown::Item>,
    md_button: Vec<markdown::Item>,
    md_text_input: Vec<markdown::Item>,
    md_message: Vec<markdown::Item>,
    md_update: Vec<markdown::Item>,
    md_tasks: Vec<markdown::Item>,
    md_subscriptions: Vec<markdown::Item>,
}

impl Default for App {
    fn default() -> Self {
        let model = UrlAnalyzer::default();

        Self {
            screen: Screen::default(),
            slide_offset: Animated::new(SlideOffset::settled(), Motion::SNAPPY),
            model,
            theme: Theme::GruvboxLight,
            ctrl_held: false,
            shift_held: false,
            chaos_circles: Vec::new(),
            button_clicks: 0,
            input_changes: 0,
            input_submits: 0,
            demo_input: String::new(),
            quiz_answer: None,
            quiz_http_answer: None,
            quiz_button_answer: None,
            quiz_validation_answer: None,
            message_log: Vec::new(),
            md_intro: markdown::parse(MD_INTRO).collect(),
            md_model: markdown::parse(MD_MODEL).collect(),
            md_view: markdown::parse(MD_VIEW).collect(),
            md_row_col: markdown::parse(MD_ROW_COL).collect(),
            md_container: markdown::parse(MD_CONTAINER).collect(),
            md_spacing: markdown::parse(MD_SPACING).collect(),
            md_button: markdown::parse(MD_BUTTON).collect(),
            md_text_input: markdown::parse(MD_TEXT_INPUT).collect(),
            md_message: markdown::parse(MD_MESSAGE).collect(),
            md_update: markdown::parse(MD_UPDATE).collect(),
            md_tasks: markdown::parse(MD_TASKS).collect(),
            md_subscriptions: markdown::parse(MD_SUBSCRIPTIONS).collect(),
        }
    }
}

const MD_INTRO: &str = r#"
The **Elm Architecture** is a pattern for structuring interactive applications.

It separates concerns into four distinct parts:

1. **Model** — the application state
2. **Message** — events from user input or the system
3. **Update** — a function that applies messages to the model
4. **View** — transforms state into UI with event handlers
"#;

const MD_MODEL: &str = r#"
```rust
enum Mode {
    Title,
    DownloadTime,
    DownloadSize,
}

struct UrlAnalyzer {
    url: String,
    secure: bool,
    mode: Mode,
}
```
"#;

const MD_BUTTON: &str = r#"
```rust
button("Get").on_press(Message::Action)
```
"#;

const MD_TEXT_INPUT: &str = r#"
```rust
text_input("Enter URL (e.g. example.com)", &self.model.url)
    .on_input(Message::UrlChanged)
    .on_submit(Message::Action)
```
"#;

const MD_MESSAGE: &str = r#"
```rust
enum Message {
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),
}
```
"#;

const MD_UPDATE: &str = r#"
```rust
fn update(&mut self, message: Message) {
    match message {
        Message::UrlChanged(url) => self.url = url,
        Message::SecureChanged(secure) => self.secure = secure,
        Message::ModeChanged(mode) => self.mode = mode,
        Message::Action => todo!("Start fetching URL"),
        Message::Result(result) => self.result = result,
    }
}
```
"#;

const MD_TASKS: &str = r#"
```rust
fn update(&mut self, message: Message) -> Task<Message> {
    ...
    Message::Action => {
        return Task::perform(
            fetch_url(self.url.clone(), self.secure, self.mode),
            Message::Result,
        );
    }
    ...
}
```
"#;

const MD_VIEW: &str = r#"
```rust
fn view(&self) -> Element<Message> {
    column![
        text_input("URL", &self.url)
            .on_input(Message::UrlChanged),
        button("Get").on_press(Message::Action),
    ].into()
}
```
"#;

const MD_ROW_COL: &str = r#"
```rust
// Horizontal layout
row![widget_a, widget_b, widget_c]

// Nested layouts
column![
    row![label, text_input],
    row![cancel_btn, submit_btn],
]
```
"#;

const MD_CONTAINER: &str = r#"
```rust
// Wrap content for positioning and styling
container(content)
    .center_x(Fill)
    .center_y(Fill)
    .padding(20)
    .style(container::rounded_box)
```
"#;

const MD_SPACING: &str = r#"
```rust
column![a, b, c]
    .spacing(10)           // Gap between children
    .padding(20)           // Space around the column
    .align_x(Center)       // Horizontal alignment
```
"#;

const MD_SUBSCRIPTIONS: &str = r#"
```rust
fn subscription(&self) -> Subscription<Message> {
    event::listen_with(|event, _, _| match event {
        Event::Keyboard(KeyPressed {
            key: Key::Named(Named::ArrowRight), ..
        }) => Some(Message::NextScreen),
        ...
    })
}
```
"#;

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NextScreen,
    PrevScreen,

    // Model updates
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),

    // Demo
    ButtonClicked,
    DemoInputChanged(String),
    DemoInputSubmitted,

    // Animation
    SlideOffset(iced_anim::Event<SlideOffset>),

    // Theme
    ThemeChanged(Theme),
    CtrlPressed,
    CtrlReleased,
    ShiftPressed,
    ShiftReleased,

    // Chaos
    SpawnChaos,
    Tick,

    // Quiz
    QuizAnswer(u8),
    QuizHttpAnswer(u8),
    QuizButtonAnswer(u8),
    QuizValidationAnswer(u8),
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Iced Tutorial")
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .font(LUCIDE_FONT_BYTES)
        .font(include_bytes!("../fonts/Bitter-Regular.ttf"))
        .font(include_bytes!("../fonts/FiraMono-Regular.ttf"))
        .default_font(BITTER)
        .run()
}

impl App {
    fn log_message(&mut self, msg: String) {
        self.message_log.push(msg);
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::Key;
        use keyboard::key::Named;

        let keyboard = event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlPressed),
            Event::Keyboard(keyboard::Event::KeyReleased {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlReleased),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Shift),
                ..
            }) => Some(Message::ShiftPressed),
            Event::Keyboard(keyboard::Event::KeyReleased {
                key: Key::Named(Named::Shift),
                ..
            }) => Some(Message::ShiftReleased),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowLeft),
                ..
            }) => Some(Message::PrevScreen),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowRight),
                ..
            }) => Some(Message::NextScreen),
            _ => None,
        });

        // Animation tick (always runs)
        let tick = iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);

        // Spawn chaos circles on the subscriptions screen
        if self.screen == Screen::Subscriptions {
            let spawn_timer =
                iced::time::every(std::time::Duration::from_secs(3)).map(|_| Message::SpawnChaos);
            Subscription::batch([keyboard, tick, spawn_timer])
        } else {
            Subscription::batch([keyboard, tick])
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Navigation
            Message::NextScreen => {
                if !self.screen.is_last() {
                    self.screen.forward();
                    self.slide_offset =
                        Animated::new(SlideOffset::entering_forward(), Motion::SNAPPY);
                    self.slide_offset.set_target(SlideOffset::settled());
                }
                Task::none()
            }
            Message::PrevScreen => {
                if !self.screen.is_first() {
                    self.screen.backward();
                    self.slide_offset =
                        Animated::new(SlideOffset::entering_backward(), Motion::SNAPPY);
                    self.slide_offset.set_target(SlideOffset::settled());
                }
                Task::none()
            }
            Message::SlideOffset(event) => {
                self.slide_offset.update(event);
                Task::none()
            }

            // Model updates
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
            Message::ButtonClicked => {
                self.button_clicks += 1;
                Task::none()
            }
            Message::DemoInputChanged(value) => {
                self.demo_input = value;
                self.input_changes += 1;
                Task::none()
            }
            Message::DemoInputSubmitted => {
                self.input_submits += 1;
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::CtrlPressed => {
                self.ctrl_held = true;
                Task::none()
            }
            Message::CtrlReleased => {
                self.ctrl_held = false;
                Task::none()
            }
            Message::ShiftPressed => {
                self.shift_held = true;
                Task::none()
            }
            Message::ShiftReleased => {
                self.shift_held = false;
                Task::none()
            }
            Message::SpawnChaos => {
                self.chaos_circles
                    .push(chaos::ChaosCircle::random(800.0, 600.0));
                Task::none()
            }
            Message::Tick => {
                for circle in &mut self.chaos_circles {
                    circle.update(800.0, 600.0);
                }
                Task::none()
            }
            Message::QuizAnswer(answer) => {
                self.quiz_answer = Some(answer);
                Task::none()
            }
            Message::QuizHttpAnswer(answer) => {
                self.quiz_http_answer = Some(answer);
                Task::none()
            }
            Message::QuizButtonAnswer(answer) => {
                self.quiz_button_answer = Some(answer);
                Task::none()
            }
            Message::QuizValidationAnswer(answer) => {
                self.quiz_validation_answer = Some(answer);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text(self.screen.to_string())
            .size(28)
            .font(FIRA_MONO)
            .color(ORANGE);

        let content: Element<Message> = match self.screen {
            Screen::Title => self.view_title_screen(),
            Screen::Intro => self.view_intro_screen(),
            Screen::Model => self.view_model_screen(),
            Screen::View => self.view_view_screen(),
            Screen::LayoutRowCol => self.view_layout_row_col_screen(),
            Screen::LayoutContainer => self.view_layout_container_screen(),
            Screen::LayoutSpacing => self.view_layout_spacing_screen(),
            Screen::Button => self.view_button_screen(),
            Screen::TextInput => self.view_text_input_screen(),
            Screen::Message => self.view_message_screen(),
            Screen::Update => self.view_update_screen(),
            Screen::Tasks => self.view_tasks_screen(),
            Screen::Subscriptions => self.view_subscriptions_screen(),
            Screen::Interactive => self.view_interactive_screen(),
            Screen::Quiz => self.view_quiz_screen(),
            Screen::QuizHttp => self.view_quiz_http_screen(),
            Screen::QuizButton => self.view_quiz_button_screen(),
            Screen::QuizValidation => self.view_quiz_validation_screen(),
            Screen::Recap => self.view_recap_screen(),
        };

        let nav = self.view_navigation();
        let nav_bar = container(nav).center_x(iced::Fill).padding(20);

        // Orange stripe at the top
        let orange_stripe =
            container(space().height(6))
                .width(iced::Fill)
                .style(|_| container::Style {
                    background: Some(ORANGE.into()),
                    ..Default::default()
                });

        let offset = self.slide_offset.value();
        let main_content = container(
            column![title, content]
                .spacing(20)
                .padding(30)
                .width(iced::Fill),
        )
        .padding(Padding {
            left: offset.left,
            right: offset.right,
            ..Padding::ZERO
        });

        let animated_content: Element<'_, Message> =
            Animation::new(&self.slide_offset, main_content)
                .on_update(Message::SlideOffset)
                .into();

        let layout = column![
            orange_stripe,
            container(animated_content).height(iced::Fill),
            nav_bar
        ];

        container(layout)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_navigation(&self) -> Element<'_, Message> {
        let prev_btn = if self.screen.is_first() {
            button("< Previous")
        } else {
            button("< Previous").on_press(Message::PrevScreen)
        };

        let next_btn = if self.screen.is_last() {
            button("Next >")
        } else {
            button("Next >").on_press(Message::NextScreen)
        };

        // Slide indicator
        let current = self.screen as usize;
        let total = Screen::COUNT;
        let slide_indicator = text(format!("{} / {}", current + 1, total))
            .size(14)
            .color(SUBTITLE_COLOR);

        let mut nav_row = row![prev_btn, slide_indicator, next_btn]
            .spacing(20)
            .align_y(iced::Alignment::Center);

        if self.ctrl_held {
            let theme_picker = row![
                text("Theme: "),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
            ]
            .spacing(10);
            nav_row = nav_row.push(theme_picker);
        }

        nav_row.into()
    }

    fn view_title_screen(&self) -> Element<'_, Message> {
        container(
            column![
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(96)
                    .height(96),
                space().height(30),
                text("Discover Iced").size(56).color(ORANGE),
                space().height(16),
                text("A quick tour of Iced (done with Iced)")
                    .size(22)
                    .color(SUBTITLE_COLOR),
                space().height(50),
                text("Use arrow keys to navigate")
                    .size(16)
                    .color(SUBTITLE_COLOR),
            ]
            .width(iced::Fill)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn md_settings(&self) -> markdown::Settings {
        let mut settings = markdown::Settings::with_text_size(TEXT_SIZE, self.theme.clone());
        settings.code_size = CODE_SIZE.into();
        settings
    }

    fn view_intro_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                self.md_container(&self.md_intro),
                space().height(30),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(260),
                space().height(30),
            ]
            .align_x(iced::Alignment::Center)
            .padding(Padding::new(20.0).left(40.0).right(40.0)),
        )
        .into()
    }

    fn md_container<'a>(&self, md: &'a [markdown::Item]) -> Element<'a, Message> {
        let md_view: Element<'a, Message, AppTheme, _> =
            markdown::view(md, self.md_settings()).map(|_| Message::Action);
        themer(Some(AppTheme(self.theme.clone())), md_view).into()
    }

    fn view_model_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_model),
                text("").size(12),
                text("Notice: completely UI-agnostic.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_view),
                space().height(12),
                text("Notice the method signature: &self (immutable borrow).").size(TEXT_SIZE),
                space().height(8),
                text("The View can read state but never modify it.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_layout_row_col_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Rows and columns are the building blocks of layout.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_row_col),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    column![
                        row![
                            text("Row 1, Col A"),
                            text("Row 1, Col B"),
                            text("Row 1, Col C"),
                        ]
                        .spacing(20),
                        row![text("Row 2, Col A"), text("Row 2, Col B")].spacing(20),
                    ]
                    .spacing(10)
                )
                .padding(15)
                .style(container::rounded_box),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_layout_container_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_container),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    container(text("Centered and styled"))
                        .padding(20)
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_layout_spacing_screen(&self) -> Element<'_, Message> {
        let row: Element<'_, Message> = row![
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(5)
                    .align_x(iced::Alignment::Start),
            )
            .padding(10)
            .style(container::rounded_box),
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(15)
                    .align_x(iced::Alignment::Center),
            )
            .padding(10)
            .style(container::rounded_box),
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(25)
                    .align_x(iced::Alignment::End),
            )
            .padding(10)
            .style(container::rounded_box),
        ]
        .spacing(20)
        .into();
        let row = if self.shift_held {
            row.explain(Color::from_rgb(0.4, 0.2, 0.8))
        } else {
            row
        };

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_spacing),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                row,
                text("hint: press shift")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_button_screen(&self) -> Element<'_, Message> {
        let click_text = if self.button_clicks == 0 {
            String::from("Click the button!")
        } else {
            format!(
                "Clicked {} time{}",
                self.button_clicks,
                if self.button_clicks == 1 { "" } else { "s" }
            )
        };

        scrollable(
            column![
                text("The Button widget produces messages when clicked.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_button),
                space().height(20),
                row![
                    button("Get").on_press(Message::ButtonClicked),
                    text(click_text).size(TEXT_SIZE),
                ]
                .align_y(iced::Center)
                .spacing(15)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_text_input_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Text Input widget produces messages as the user types.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_text_input),
                space().height(20),
                text_input("Enter URL (e.g. example.com)", &self.demo_input)
                    .on_input(Message::DemoInputChanged)
                    .on_submit(Message::DemoInputSubmitted),
                space().height(12),
                text!("Input Changed messages: {}", self.input_changes).size(TEXT_SIZE),
                text!("Input Submitted messages: {}", self.input_submits).size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_message_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(TEXT_SIZE),
                text("").size(8),
                self.md_container(&self.md_message),
                text("").size(12),
                text("Messages are produced by the view.").size(TEXT_SIZE)
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_update_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(TEXT_SIZE),
                text("").size(8),
                self.md_container(&self.md_update),
                text("").size(12),
                text("Notice the method signature! (&mut)").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_tasks_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The update function may produce a Task for async background operations.")
                    .size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_tasks),
                space().height(12),
                text("Task::perform takes an async function and a message constructor.")
                    .size(TEXT_SIZE),
                space().height(8),
                text(
                    "When the async work completes, the result is usually wrapped in the message."
                )
                .size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_subscriptions_screen(&self) -> Element<'_, Message> {
        let content = scrollable(
            column![
                text("Subscriptions let your app react to external events.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_subscriptions),
                space().height(16),
                space().height(8),
                text("  • Arrow Right → next slide")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                text("  • Arrow Left → previous slide")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                text("  • Ctrl → show theme picker")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                space().height(16),
                text("Other common uses: timers, window events, WebSocket messages.")
                    .size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        );

        let chaos_overlay = canvas(chaos::ChaosOverlay {
            circles: &self.chaos_circles,
        })
        .width(iced::Fill)
        .height(iced::Fill);

        stack![content, chaos_overlay].into()
    }

    fn view_interactive_screen(&self) -> Element<'_, Message> {
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
                .size(14)
                .color(SUBTITLE_COLOR)
                .into()
        } else {
            column(
                self.message_log
                    .iter()
                    .map(|msg| {
                        row![
                            text(msg).size(14).font(FIRA_MONO),
                            space().width(Length::Fill)
                        ]
                        .into()
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
            text(result_text).size(TEXT_SIZE),
            space().height(36),
            // State and messages side by side (half width each)
            row![
                column![
                    text("Current State")
                        .size(16)
                        .font(FIRA_MONO)
                        .color(SUBTITLE_COLOR),
                    space().height(8),
                    container(text(state_ron).size(14).font(FIRA_MONO))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(iced::Length::FillPortion(1)),
                column![
                    text("Recent Messages")
                        .size(16)
                        .font(FIRA_MONO)
                        .color(SUBTITLE_COLOR),
                    space().height(8),
                    container(scrollable(message_log_content).height(150))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(iced::Length::FillPortion(1)),
            ]
            .spacing(20),
        ]
        .width(iced::Fill)
        .into()
    }

    fn view_quiz_screen(&self) -> Element<'_, Message> {
        let correct_color = Color::from_rgb(0.18, 0.65, 0.35);
        let incorrect_color = Color::from_rgb(0.85, 0.25, 0.25);

        let feedback: Element<'_, Message> = match self.quiz_answer {
            None => text("Select an answer above")
                .size(16)
                .color(SUBTITLE_COLOR)
                .into(),
            Some(2) => text("Correct! The Update function processes input and validates data before updating the Model.")
                .size(18)
                .color(correct_color)
                .into(),
            Some(0) => text("Not quite. The View only renders UI from state — it shouldn't contain logic.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(1) => text("Not quite. Messages are just data describing what happened — they don't contain logic.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(3) => text("Not quite. The Model only holds state, not logic.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(_) => space().into(),
        };

        container(
            column![
                text("Where should validation of a text input happen?")
                    .size(32)
                    .color(ORANGE),
                space().height(30),
                column![
                    button("A) In the View").on_press(Message::QuizAnswer(0)),
                    button("B) In the Message").on_press(Message::QuizAnswer(1)),
                    button("C) In the Update").on_press(Message::QuizAnswer(2)),
                    button("D) In the Model").on_press(Message::QuizAnswer(3)),
                ]
                .spacing(12),
                space().height(25),
                feedback,
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn view_quiz_http_screen(&self) -> Element<'_, Message> {
        let correct_color = Color::from_rgb(0.18, 0.65, 0.35);
        let incorrect_color = Color::from_rgb(0.85, 0.25, 0.25);

        let feedback: Element<'_, Message> = match self.quiz_http_answer {
            None => text("Select an answer above")
                .size(16)
                .color(SUBTITLE_COLOR)
                .into(),
            Some(2) => text("Correct! HTTP requests are async operations, so they belong in a Task returned from Update.")
                .size(18)
                .color(correct_color)
                .into(),
            Some(0) => text("Not quite. The View only renders UI — it can't perform side effects.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(1) => text("Not quite. Messages are just data — they describe events, not perform actions.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(3) => text("Not quite. The Model only holds state — it doesn't perform operations.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(_) => space().into(),
        };

        container(
            column![
                text("Where should you make an HTTP request?")
                    .size(32)
                    .color(ORANGE),
                space().height(30),
                column![
                    button("A) In the View").on_press(Message::QuizHttpAnswer(0)),
                    button("B) In the Message").on_press(Message::QuizHttpAnswer(1)),
                    button("C) In a Task returned from Update")
                        .on_press(Message::QuizHttpAnswer(2)),
                    button("D) In the Model").on_press(Message::QuizHttpAnswer(3)),
                ]
                .spacing(12),
                space().height(25),
                feedback,
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn view_quiz_button_screen(&self) -> Element<'_, Message> {
        let correct_color = Color::from_rgb(0.18, 0.65, 0.35);
        let incorrect_color = Color::from_rgb(0.85, 0.25, 0.25);

        let feedback: Element<'_, Message> = match self.quiz_button_answer {
            None => text("Select an answer above")
                .size(16)
                .color(SUBTITLE_COLOR)
                .into(),
            Some(0) => text("Correct! For simple conditions, the View can check directly with conditional on_press.")
                .size(18)
                .color(correct_color)
                .into(),
            Some(1) => text("Correct! For complex logic, Update can set a flag in the Model that the View reads.")
                .size(18)
                .color(correct_color)
                .into(),
            Some(2) => text("Not quite. Messages don't control UI state — they describe events.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(_) => space().into(),
        };

        container(
            column![
                text("How do you disable a button when a field is empty?")
                    .size(32)
                    .color(ORANGE),
                space().height(30),
                column![
                    button("A) View checks the condition with conditional on_press")
                        .on_press(Message::QuizButtonAnswer(0)),
                    button("B) Update sets a flag in the Model, View reads it")
                        .on_press(Message::QuizButtonAnswer(1)),
                    button("C) Send a DisableButton message")
                        .on_press(Message::QuizButtonAnswer(2)),
                ]
                .spacing(12),
                space().height(25),
                feedback,
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn view_quiz_validation_screen(&self) -> Element<'_, Message> {
        let correct_color = Color::from_rgb(0.18, 0.65, 0.35);
        let incorrect_color = Color::from_rgb(0.85, 0.25, 0.25);

        let feedback: Element<'_, Message> = match self.quiz_validation_answer {
            None => text("Select an answer above")
                .size(16)
                .color(SUBTITLE_COLOR)
                .into(),
            Some(0) => text("Correct! Update validates and stores errors in the Model. The View reads those errors and displays them. Messages carry the input data.")
                .size(18)
                .color(correct_color)
                .into(),
            Some(1) => text("Not quite. The View shouldn't contain validation logic — it only renders based on Model state.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(2) => text("Not quite. While Update does the validation, the error must be stored in the Model for the View to display it.")
                .size(18)
                .color(incorrect_color)
                .into(),
            Some(_) => space().into(),
        };

        container(
            column![
                text("How does input validation with error display work?")
                    .size(32)
                    .color(ORANGE),
                space().height(30),
                column![
                    button("A) Update validates, stores error in Model, View displays it")
                        .on_press(Message::QuizValidationAnswer(0)),
                    button("B) View validates and shows error directly")
                        .on_press(Message::QuizValidationAnswer(1)),
                    button("C) Update validates and shows error directly")
                        .on_press(Message::QuizValidationAnswer(2)),
                ]
                .spacing(12),
                space().height(25),
                feedback,
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn view_recap_screen(&self) -> Element<'_, Message> {
        container(
            column![
                text("The Elm Architecture").size(40).color(ORANGE),
                space().height(30),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(280),
                space().height(30),
                text("Model → View → Message → Update → (Task) → Model...")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
                space().height(20),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}

async fn fetch_url(url: String, secure: bool, mode: Mode) -> String {
    let protocol = if secure { "https" } else { "http" };
    let full_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("{protocol}://{url}")
    };

    let start = std::time::Instant::now();

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

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

            // Extract title from HTML
            if let Some(start_idx) = body.find("<title>")
                && let Some(end_idx) = body.find("</title>")
            {
                let title = &body[start_idx + 7..end_idx];
                return format!("Title: {}", title.trim());
            }
            "No <title> found".to_string()
        }
        Mode::DownloadTime => {
            // Consume the body to measure full download time
            match response.bytes().await {
                Ok(_) => format!("Download time: {elapsed:.2?}"),
                Err(error) => format!("Error: {error}"),
            }
        }
        Mode::DownloadSize => match response.bytes().await {
            Ok(bytes) => format!(
                "Size: {}",
                humansize::format_size(bytes.len(), humansize::DECIMAL)
            ),
            Err(error) => format!("Error: {error}"),
        },
    }
}
