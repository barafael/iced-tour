use iced::{
    Color, Element, Event, Font, Padding, Subscription, Task, Theme, event, keyboard,
    widget::{canvas, column, container, markdown, pick_list, row, space, stack, text, themer},
};
use iced_anim::{Animation, widget::button};
use lucide_icons::{
    LUCIDE_FONT_BYTES,
    iced::{icon_chevron_left, icon_chevron_right},
};
use strum::EnumCount;

use crate::slides::Slide;

use theme::AppTheme;

mod chaos;
mod demo;
mod navigation;
mod quiz;
mod slides;
mod sliding;
mod terminal;
mod theme;
mod theming;

pub const BITTER: Font = Font::with_name("Bitter");
pub const FIRA_MONO: Font = Font::with_name("Fira Mono");

pub const ICED_LOGO: &[u8] = include_bytes!("../assets/iced-logo.svg");

pub const TEXT_SIZE: u32 = 24;
const CODE_SIZE: u32 = 22;

pub const ORANGE: Color = Color::from_rgb(1.0, 0.4, 0.0);
pub const SUBTITLE_COLOR: Color = Color::from_rgb(0.45, 0.45, 0.45);
pub const CORRECT_COLOR: Color = Color::from_rgb(0.18, 0.65, 0.35);
pub const INCORRECT_COLOR: Color = Color::from_rgb(0.85, 0.25, 0.25);

pub const ELM_CIRCLE_OF_LIFE: &[u8] = include_bytes!("../assets/elm.svg");

pub struct App {
    pub navigation: navigation::Navigation,
    pub demo: demo::Demo,
    pub theming: theming::Theming,
    pub chaos: chaos::Chaos,
    pub quiz: quiz::Quiz,
    pub page_boop: page_boop::PageBoop,
    pub terminal: terminal::Terminal,
    pub ctrl_held: bool,
    pub shift_held: bool,

    // Cached markdown content for each screen
    pub md_intro: Vec<markdown::Item>,
    pub md_model: Vec<markdown::Item>,
    pub md_view: Vec<markdown::Item>,
    pub md_row_col: Vec<markdown::Item>,
    pub md_container: Vec<markdown::Item>,
    pub md_spacing: Vec<markdown::Item>,
    pub md_button: Vec<markdown::Item>,
    pub md_text_input: Vec<markdown::Item>,
    pub md_message: Vec<markdown::Item>,
    pub md_update: Vec<markdown::Item>,
    pub md_tasks: Vec<markdown::Item>,
    pub md_subscriptions: Vec<markdown::Item>,
    pub md_constructors: Vec<markdown::Item>,
    pub md_widget_messages: Vec<markdown::Item>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            navigation: navigation::Navigation::default(),
            demo: demo::Demo::default(),
            theming: theming::Theming::default(),
            chaos: chaos::Chaos::default(),
            quiz: crate::quiz::Quiz::default(),
            page_boop: page_boop::PageBoop::with_style(page_boop::StyleConfig {
                mono_font: FIRA_MONO,
                subtitle_color: SUBTITLE_COLOR,
                text_size: TEXT_SIZE,
            }),
            terminal: terminal::Terminal::new(FIRA_MONO),
            ctrl_held: false,
            shift_held: false,
            md_intro: markdown::parse(slides::intro::MD_INTRO).collect(),
            md_model: markdown::parse(slides::model::MD_MODEL).collect(),
            md_view: markdown::parse(slides::view::MD_VIEW).collect(),
            md_row_col: markdown::parse(slides::layout::MD_ROW_COL).collect(),
            md_container: markdown::parse(slides::layout::MD_CONTAINER).collect(),
            md_spacing: markdown::parse(slides::layout::MD_SPACING).collect(),
            md_button: markdown::parse(slides::button::MD_BUTTON).collect(),
            md_text_input: markdown::parse(slides::text_input::MD_TEXT_INPUT).collect(),
            md_message: markdown::parse(slides::message::MD_MESSAGE).collect(),
            md_update: markdown::parse(slides::update::MD_UPDATE).collect(),
            md_tasks: markdown::parse(slides::tasks::MD_TASKS).collect(),
            md_subscriptions: markdown::parse(slides::subscriptions::MD_SUBSCRIPTIONS).collect(),
            md_constructors: markdown::parse(slides::constructors::MD_CONSTRUCTORS).collect(),
            md_widget_messages: markdown::parse(slides::constructors::MD_WIDGET_MESSAGES).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(navigation::Message),
    Demo(demo::Message),
    Theming(theming::Message),
    Chaos(chaos::Message),
    Quiz(quiz::Message),
    PageBoop(page_boop::Message),
    Terminal(terminal::Message),

    CtrlPressed,
    CtrlReleased,
    ShiftPressed,
    ShiftReleased,

    Noop,
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Iced Tutorial")
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .font(LUCIDE_FONT_BYTES)
        .font(iced_aw::ICED_AW_FONT_BYTES)
        .font(include_bytes!("../fonts/Bitter-Regular.ttf"))
        .font(include_bytes!("../fonts/FiraMono-Regular.ttf"))
        .default_font(BITTER)
        .run()
}

impl App {
    fn theme(&self) -> Theme {
        self.theming.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::Key;
        use keyboard::key::Named;

        let events = event::listen_with(|event, _status, _id| match event {
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
            }) => Some(Message::Navigation(navigation::Message::PrevScreen)),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowRight),
                ..
            }) => Some(Message::Navigation(navigation::Message::NextScreen)),
            Event::Window(iced::window::Event::Resized(size)) => Some(Message::Chaos(
                chaos::Message::WindowResized(size.width, size.height),
            )),
            _ => None,
        });

        let screen = self.navigation.screen;
        let needs_tick = screen == Slide::Subscriptions || self.navigation.is_animating();

        let term_sub = self.terminal.subscription().map(Message::Terminal);

        if screen == Slide::Subscriptions {
            let tick = iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Chaos(chaos::Message::Tick));
            let spawn_timer = iced::time::every(std::time::Duration::from_secs(3))
                .map(|_| Message::Chaos(chaos::Message::SpawnChaos));
            Subscription::batch([events, tick, spawn_timer, term_sub])
        } else if needs_tick {
            let tick = iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Chaos(chaos::Message::Tick));
            Subscription::batch([events, tick, term_sub])
        } else {
            Subscription::batch([events, term_sub])
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),

            Message::Navigation(msg) => {
                match self.navigation.update(msg) {
                    navigation::Action::None => {}
                    navigation::Action::SlideChanged => {
                        self.chaos.clear_and_unpause();
                    }
                }
                Task::none()
            }

            Message::Demo(msg) => {
                match self.demo.update(msg) {
                    demo::Action::None => {}
                }
                Task::none()
            }

            Message::Theming(msg) => {
                match self.theming.update(msg) {
                    theming::Action::None => {}
                    theming::Action::ThemeChanged(_) => {}
                }
                Task::none()
            }

            Message::Chaos(msg) => {
                match self.chaos.update(msg) {
                    chaos::Action::None => {}
                }
                Task::none()
            }

            Message::Quiz(msg) => {
                match self.quiz.update(msg) {
                    quiz::Action::None => {}
                }
                Task::none()
            }

            Message::PageBoop(msg) => match self.page_boop.update(msg) {
                page_boop::Action::None => Task::none(),
                page_boop::Action::Run(task) => task.map(Message::PageBoop),
            },

            Message::Terminal(msg) => {
                match self.terminal.update(msg) {
                    terminal::Action::None => {}
                }
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
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let screen = self.navigation.screen;

        let title = text(screen.to_string())
            .size(self.sz(38))
            .font(FIRA_MONO)
            .color(ORANGE);

        let content: Element<Message> = match screen {
            Slide::Title => self.view_title_screen(),
            Slide::Intro => self.view_intro_screen(),
            Slide::Model => self.view_model_screen(),
            Slide::View => self.view_view_screen(),
            Slide::LayoutRowCol => self.view_layout_row_col_screen(),
            Slide::LayoutContainer => self.view_layout_container_screen(),
            Slide::LayoutSpacing => self.view_layout_spacing_screen(),
            Slide::Button => self.view_button_screen(),
            Slide::TextInput => self.view_text_input_screen(),
            Slide::Theming => self.view_theming_screen(),
            Slide::ThemePicker => self.view_theme_picker_screen(),
            Slide::Message => self.view_message_screen(),
            Slide::Constructors => self.view_constructors_screen(),
            Slide::Update => self.view_update_screen(),
            Slide::Tasks => self.view_tasks_screen(),
            Slide::Subscriptions => self.view_subscriptions_screen(),
            Slide::Interactive => self.view_interactive_screen(),
            Slide::CommunityWidgets => self.view_community_widgets_screen(),
            Slide::Quiz => self.view_quiz_screen(),
            Slide::QuizHttp => self.view_quiz_http_screen(),
            Slide::QuizButton => self.view_quiz_button_screen(),
            Slide::QuizValidation => self.view_quiz_validation_screen(),
            Slide::Takeaways => self.view_takeaways_screen(),
            Slide::Recap => self.view_recap_screen(),
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

        let offset = self.navigation.slide_offset.value();
        let main_content = container(
            column![title, content]
                .spacing(self.sp(20.0))
                .padding(self.sp(30.0))
                .width(iced::Fill),
        )
        .padding(Padding {
            left: offset.left,
            right: offset.right,
            ..Padding::ZERO
        });

        let animated_content: Element<'_, Message> =
            Animation::new(&self.navigation.slide_offset, main_content)
                .on_update(|event| Message::Navigation(navigation::Message::SlideOffset(event)))
                .into();

        let layout = column![
            orange_stripe,
            container(animated_content).height(iced::Fill),
            nav_bar
        ];

        if screen == Slide::Subscriptions {
            let chaos_overlay = canvas(chaos::ChaosOverlay {
                circles: &self.chaos.circles,
            })
            .width(iced::Fill)
            .height(iced::Fill);

            container(stack![layout, chaos_overlay])
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        } else {
            container(layout)
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        }
    }

    fn view_navigation(&self) -> Element<'_, Message> {
        let screen = self.navigation.screen;

        let prev_label = row![icon_chevron_left(), text("Previous")]
            .spacing(4)
            .align_y(iced::Alignment::Center);
        let next_label = row![text("Next"), icon_chevron_right()]
            .spacing(4)
            .align_y(iced::Alignment::Center);

        let prev_btn = if screen.is_first() {
            button(prev_label)
        } else {
            button(prev_label).on_press(Message::Navigation(navigation::Message::PrevScreen))
        };

        let next_btn = if screen.is_last() {
            button(next_label)
        } else {
            button(next_label).on_press(Message::Navigation(navigation::Message::NextScreen))
        };

        // Slide indicator
        let current = screen as usize;
        let total = Slide::COUNT;
        let slide_indicator = text(format!("{} / {}", current + 1, total))
            .size(self.sz(20))
            .color(SUBTITLE_COLOR);

        let mut nav_row = row![prev_btn, slide_indicator, next_btn]
            .spacing(20)
            .align_y(iced::Alignment::Center);

        if self.ctrl_held {
            let theme_picker = row![
                text("Theme: "),
                pick_list(Theme::ALL, Some(&self.theming.theme), |t| Message::Theming(
                    theming::Message::ThemeChanged(t)
                ),),
            ]
            .spacing(10);
            nav_row = nav_row.push(theme_picker);
        }

        nav_row.into()
    }

    /// Scale factor based on window size relative to 1024x768 base.
    pub fn scale(&self) -> f32 {
        self.chaos.scale()
    }

    /// Scale a pixel size (for text sizes, icon sizes, etc).
    pub fn sz(&self, base: u32) -> u32 {
        ((base as f32) * self.scale()) as u32
    }

    /// Scale a float value (for spacing, padding, heights).
    pub fn sp(&self, base: f32) -> f32 {
        base * self.scale()
    }

    pub fn md_settings(&self) -> markdown::Settings {
        let mut settings =
            markdown::Settings::with_text_size(self.sz(TEXT_SIZE), self.theming.theme.clone());
        settings.code_size = self.sz(CODE_SIZE).into();
        settings
    }

    pub fn md_container<'a>(&self, md: &'a [markdown::Item]) -> Element<'a, Message> {
        let md_view: Element<'a, Message, AppTheme, _> =
            markdown::view(md, self.md_settings()).map(|_| Message::Noop);
        themer(Some(AppTheme(self.theming.theme.clone())), md_view).into()
    }
}
