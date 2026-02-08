use iced_anim::Animate;

#[derive(Debug, Clone, PartialEq, Animate)]
pub struct SlideOffset {
    pub left: f32,
    pub right: f32,
}

impl SlideOffset {
    pub fn settled() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn entering_forward() -> Self {
        Self {
            left: 20.0,
            right: 0.0,
        }
    }

    pub fn entering_backward() -> Self {
        Self {
            left: 0.0,
            right: 20.0,
        }
    }
}
