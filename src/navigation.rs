use crate::slides::Slide;
use crate::sliding;
use iced_anim::{Animated, Motion};

pub struct Navigation {
    screen: Slide,
    slide_offset: Animated<sliding::SlideOffset>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextScreen,
    PrevScreen,
    SlideOffset(iced_anim::Event<sliding::SlideOffset>),
}

pub enum Action {
    None,
    SlideChanged,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            screen: Slide::default(),
            slide_offset: Animated::new(sliding::SlideOffset::settled(), Motion::SNAPPY),
        }
    }
}

impl Navigation {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NextScreen => {
                if !self.screen.is_last() {
                    self.screen.forward();
                    self.slide_offset =
                        Animated::new(sliding::SlideOffset::entering_forward(), Motion::SNAPPY);
                    self.slide_offset
                        .set_target(sliding::SlideOffset::settled());
                    Action::SlideChanged
                } else {
                    Action::None
                }
            }
            Message::PrevScreen => {
                if !self.screen.is_first() {
                    self.screen.backward();
                    self.slide_offset =
                        Animated::new(sliding::SlideOffset::entering_backward(), Motion::SNAPPY);
                    self.slide_offset
                        .set_target(sliding::SlideOffset::settled());
                    Action::SlideChanged
                } else {
                    Action::None
                }
            }
            Message::SlideOffset(event) => {
                self.slide_offset.update(event);
                Action::None
            }
        }
    }

    pub fn screen(&self) -> Slide {
        self.screen
    }

    pub fn slide_offset(&self) -> &Animated<sliding::SlideOffset> {
        &self.slide_offset
    }

    pub fn is_animating(&self) -> bool {
        self.slide_offset.value() != &sliding::SlideOffset::settled()
    }
}
