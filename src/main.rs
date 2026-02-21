use std::time::Duration;

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

const TICK_INTERVAL: Duration = Duration::from_millis(16);
const CHAOS_SPAWN_INTERVAL: Duration = Duration::from_secs(3);

// --- Scaling context ---

#[derive(Clone, Copy)]
pub struct ScaleCtx {
    scale: f32,
}

impl ScaleCtx {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn sz(&self, base: u32) -> u32 {
        ((base as f32) * self.scale) as u32
    }

    pub fn sp(&self, base: f32) -> f32 {
        base * self.scale
    }
}

pub fn render_markdown<'a>(
    md: &'a [markdown::Item],
    ctx: ScaleCtx,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut settings = markdown::Settings::with_text_size(ctx.sz(TEXT_SIZE), theme.clone());
    settings.code_size = ctx.sz(CODE_SIZE).into();
    let md_view: Element<'a, Message, AppTheme, _> =
        markdown::view(md, settings).map(|_| Message::Noop);
    themer(Some(AppTheme(theme.clone())), md_view).into()
}

// --- App ---

pub struct App {
    // Components
    navigation: navigation::Navigation,
    demo: demo::Demo,
    theming: theming::Theming,
    chaos: chaos::Chaos,
    quiz: quiz::Quiz,
    page_boop: page_boop::PageBoop,
    terminal: terminal::Terminal,

    // Input state
    ctrl_held: bool,
    shift_held: bool,

    // Slide state (each slide owns its cached markdown)
    title_slide: slides::title::TitleSlide,
    intro_slide: slides::intro::IntroSlide,
    model_slide: slides::model::ModelSlide,
    view_slide: slides::view::ViewSlide,
    layout_slide: slides::layout::LayoutSlide,
    button_slide: slides::button::ButtonSlide,
    text_input_slide: slides::text_input::TextInputSlide,
    message_slide: slides::message::MessageSlide,
    constructors_slide: slides::constructors::ConstructorsSlide,
    update_slide: slides::update::UpdateSlide,
    tasks_slide: slides::tasks::TasksSlide,
    subscriptions_slide: slides::subscriptions::SubscriptionsSlide,
    interactive_slide: slides::interactive::InteractiveSlide,
    community_widgets_slide: slides::community_widgets::CommunityWidgetsSlide,

    recap_slide: slides::recap::RecapSlide,
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
            title_slide: slides::title::TitleSlide,
            intro_slide: slides::intro::IntroSlide::default(),
            model_slide: slides::model::ModelSlide::default(),
            view_slide: slides::view::ViewSlide::default(),
            layout_slide: slides::layout::LayoutSlide::default(),
            button_slide: slides::button::ButtonSlide::default(),
            text_input_slide: slides::text_input::TextInputSlide::default(),
            message_slide: slides::message::MessageSlide::default(),
            constructors_slide: slides::constructors::ConstructorsSlide::default(),
            update_slide: slides::update::UpdateSlide::default(),
            tasks_slide: slides::tasks::TasksSlide::default(),
            subscriptions_slide: slides::subscriptions::SubscriptionsSlide::default(),
            interactive_slide: slides::interactive::InteractiveSlide,
            community_widgets_slide: slides::community_widgets::CommunityWidgetsSlide,
            recap_slide: slides::recap::RecapSlide,
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
        self.theming.theme().clone()
    }

    fn scale_ctx(&self) -> ScaleCtx {
        ScaleCtx::new(self.chaos.scale())
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

        let screen = self.navigation.screen();
        let term_sub = self.terminal.subscription().map(Message::Terminal);

        let mut subs = vec![events, term_sub];

        if screen == Slide::Subscriptions || self.navigation.is_animating() {
            subs.push(
                iced::time::every(TICK_INTERVAL).map(|_| Message::Chaos(chaos::Message::Tick)),
            );
        }

        if screen == Slide::Subscriptions {
            subs.push(
                iced::time::every(CHAOS_SPAWN_INTERVAL)
                    .map(|_| Message::Chaos(chaos::Message::SpawnChaos)),
            );
        }

        Subscription::batch(subs)
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
                    theming::Action::ThemeChanged => {}
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
        let screen = self.navigation.screen();
        let ctx = self.scale_ctx();
        let theme = self.theming.theme();

        let title = text(screen.to_string())
            .size(ctx.sz(38))
            .font(FIRA_MONO)
            .color(ORANGE);

        let content: Element<Message> = match screen {
            Slide::Title => self.title_slide.view(ctx),
            Slide::Intro => self.intro_slide.view(ctx, theme),
            Slide::Model => self.model_slide.view(ctx, theme),
            Slide::View => self.view_slide.view_view(ctx, theme),
            Slide::LayoutRowCol => self.layout_slide.view_row_col(ctx, theme),
            Slide::LayoutContainer => self.layout_slide.view_container(ctx, theme),
            Slide::LayoutSpacing => {
                self.layout_slide
                    .view_spacing(ctx, theme, &self.demo, self.shift_held)
            }
            Slide::Button => self.button_slide.view(ctx, theme, &self.demo),
            Slide::TextInput => self.text_input_slide.view(ctx, theme, &self.demo),
            Slide::Theming => self.view_slide.view_theming(ctx, &self.theming),
            Slide::ThemePicker => self.view_slide.view_theme_picker(ctx, &self.theming),
            Slide::Message => self.message_slide.view(ctx, theme),
            Slide::Constructors => self.constructors_slide.view(ctx, theme),
            Slide::Update => self.update_slide.view(ctx, theme),
            Slide::Tasks => self.tasks_slide.view(ctx, theme),
            Slide::Subscriptions => self.subscriptions_slide.view(ctx, theme),
            Slide::Interactive => self.interactive_slide.view(&self.page_boop),
            Slide::CommunityWidgets => self.community_widgets_slide.view(ctx, &self.terminal),
            Slide::Quiz => slides::quiz::QuizSlides::view_quiz_screen(ctx, &self.quiz),
            Slide::QuizHttp => slides::quiz::QuizSlides::view_quiz_http(ctx, &self.quiz),
            Slide::QuizButton => slides::quiz::QuizSlides::view_quiz_button(ctx, &self.quiz),
            Slide::QuizValidation => {
                slides::quiz::QuizSlides::view_quiz_validation(ctx, &self.quiz)
            }
            Slide::Takeaways => self.recap_slide.view_takeaways(ctx),
            Slide::Recap => self.recap_slide.view_recap(ctx),
        };

        let nav = self.view_navigation(ctx);
        let nav_bar = container(nav).center_x(iced::Fill).padding(20);

        // Orange stripe at the top
        let orange_stripe =
            container(space().height(6))
                .width(iced::Fill)
                .style(|_| container::Style {
                    background: Some(ORANGE.into()),
                    ..Default::default()
                });

        let offset = self.navigation.slide_offset().value();
        let main_content = container(
            column![title, content]
                .spacing(ctx.sp(20.0))
                .padding(ctx.sp(30.0))
                .width(iced::Fill),
        )
        .padding(Padding {
            left: offset.left,
            right: offset.right,
            ..Padding::ZERO
        });

        let animated_content: Element<'_, Message> =
            Animation::new(self.navigation.slide_offset(), main_content)
                .on_update(|event| Message::Navigation(navigation::Message::SlideOffset(event)))
                .into();

        let layout = column![
            orange_stripe,
            container(animated_content).height(iced::Fill),
            nav_bar
        ];

        if screen == Slide::Subscriptions {
            let chaos_overlay = canvas(chaos::ChaosOverlay {
                circles: self.chaos.circles(),
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

    fn view_navigation(&self, ctx: ScaleCtx) -> Element<'_, Message> {
        let screen = self.navigation.screen();

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
            .size(ctx.sz(20))
            .color(SUBTITLE_COLOR);

        let mut nav_row = row![prev_btn, slide_indicator, next_btn]
            .spacing(20)
            .align_y(iced::Alignment::Center);

        if self.ctrl_held {
            let theme_picker = row![
                text("Theme: "),
                pick_list(
                    Theme::ALL,
                    Some(self.theming.theme()),
                    |t| Message::Theming(theming::Message::ThemeChanged(t)),
                ),
            ]
            .spacing(10);
            nav_row = nav_row.push(theme_picker);
        }

        nav_row.into()
    }
}
