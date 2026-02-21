use iced::{
    Color, Rectangle,
    widget::canvas::{self, Frame, Path, Stroke},
};
use rand::Rng;

#[derive(Clone)]
pub struct ChaosCircle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub color: Color,
}

impl ChaosCircle {
    pub fn random(bounds_width: f32, bounds_height: f32) -> Self {
        let mut rng = rand::rng();

        let radius = rng.random_range(20.0..80.0);
        let x = rng.random_range(radius..bounds_width - radius);
        let y = rng.random_range(radius..bounds_height - radius);

        let vx = rng.random_range(-2.0..2.0);
        let vy = rng.random_range(-2.0..2.0);

        let color = Color {
            r: rng.random_range(0.0..1.0),
            g: rng.random_range(0.0..1.0),
            b: rng.random_range(0.0..1.0),
            a: rng.random_range(0.3..0.9),
        };

        Self {
            x,
            y,
            vx,
            vy,
            radius,
            color,
        }
    }

    fn update_physics(&mut self, bounds_width: f32, bounds_height: f32) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x - self.radius < 0.0 || self.x + self.radius > bounds_width {
            self.vx = -self.vx;
            self.x = self.x.clamp(self.radius, bounds_width - self.radius);
        }
        if self.y - self.radius < 0.0 || self.y + self.radius > bounds_height {
            self.vy = -self.vy;
            self.y = self.y.clamp(self.radius, bounds_height - self.radius);
        }
    }
}

pub struct ChaosOverlay<'a> {
    pub circles: &'a [ChaosCircle],
}

impl<Message> canvas::Program<Message> for ChaosOverlay<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for circle in self.circles {
            let center = iced::Point::new(circle.x, circle.y);
            let path = Path::circle(center, circle.radius);

            frame.fill(&path, circle.color);

            frame.stroke(
                &path,
                Stroke::default().with_color(Color::BLACK).with_width(2.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

// --- Composition pattern ---

pub struct Chaos {
    circles: Vec<ChaosCircle>,
    paused: bool,
    canvas_size: (f32, f32),
}

#[derive(Debug, Clone)]
pub enum Message {
    SpawnChaos,
    PanicChaos,
    Tick,
    WindowResized(f32, f32),
}

pub enum Action {
    None,
}

impl Default for Chaos {
    fn default() -> Self {
        Self {
            circles: Vec::new(),
            paused: false,
            canvas_size: (800.0, 600.0),
        }
    }
}

impl Chaos {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SpawnChaos => {
                if !self.paused {
                    let (w, h) = self.canvas_size;
                    self.circles.push(ChaosCircle::random(w, h));
                }
            }
            Message::PanicChaos => {
                self.circles.clear();
                self.paused = true;
            }
            Message::Tick => {
                let (w, h) = self.canvas_size;
                for circle in &mut self.circles {
                    circle.update_physics(w, h);
                }
            }
            Message::WindowResized(width, height) => {
                self.canvas_size = (width, height);
            }
        }
        Action::None
    }

    pub fn clear_and_unpause(&mut self) {
        self.circles.clear();
        self.paused = false;
    }

    pub fn circles(&self) -> &[ChaosCircle] {
        &self.circles
    }

    pub fn scale(&self) -> f32 {
        let (w, h) = self.canvas_size;
        (w / 1024.0).min(h / 768.0).max(0.5)
    }
}
