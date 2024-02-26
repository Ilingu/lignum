use std::f32::consts::PI;

use ggez::{
    glam::{self, vec2, Vec2},
    graphics::{self, Canvas, GraphicsContext, Image},
    mint::{ColumnMatrix4, Point2},
    GameError,
};

use crate::norm;

#[derive(Default)]
pub struct Bird {
    id: usize,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Bird {
    pub fn pos(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn orientation(&self) -> f32 {
        (self.dy).atan2(self.dx)
    }

    /// return a number between 1 and 10
    pub fn bird_frame_velocity(&self) -> usize {
        const SPEED_MEAN: f64 = 0.1;
        (9.5 * (-SPEED_MEAN * (norm!(self.dx, self.dy).max(1.0) - 1.0)).exp() + 0.5)
            .min(10.0)
            .max(1.0)
            .ceil() as usize
    }
}

pub struct State {
    // parameters
    pub separation: f32,
    pub match_factor: f32,
    pub cohesion: f32,
    pub vision_range: f32,

    // assets
    pub bird_frame: Vec<Image>,

    // Windows
    pub window_size: (u32, u32),

    // game state
    pub birds: Vec<Bird>,
    pub bird_frame_number: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            separation: 1.0,
            match_factor: 0.03,
            cohesion: 0.01,
            vision_range: 75.0,
            window_size: Default::default(),
            birds: Default::default(),
            bird_frame: Default::default(),
            bird_frame_number: Default::default(),
        }
    }
}

impl State {
    pub fn new(
        number_of_birds: usize,
        gfx: &GraphicsContext,
        width: f32,
        height: f32,
    ) -> Result<Self, GameError> {
        let mut rng = fastrand::Rng::new();

        // load bird frames
        let frames = (0..8)
            .map(|i| {
                graphics::Image::from_path(gfx, format!("/bird_frame/frame_{}.png", i)).unwrap()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            window_size: (width as u32, height as u32),
            bird_frame: frames,
            birds: (0..number_of_birds)
                .map(|id| Bird {
                    id,
                    x: rng.f32() * width,
                    y: rng.f32() * height,
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        })
    }

    pub fn update_window_size(&mut self, gfx: &GraphicsContext) {
        let current_window_size = gfx.window().inner_size();
        self.window_size = (current_window_size.width, current_window_size.height);
    }

    pub fn compute_next_frame(&mut self) {
        for bid in 0..(self.birds.len()) {
            let bird = &self.birds[bid];
            let neighbors = self.birds.iter().filter(|b| {
                b.id != bird.id && norm!(bird.x - b.x, bird.y - b.y) as f32 <= self.vision_range
            });

            let mut cohesion_point_sum = Vec2::default();
            let mut separation_force = Vec2::default();
            let mut velocity_sum = Vec2::default();
            let mut orientation_sum = 0.0;
            let mut neighbors_len: usize = 0;

            for nbird in neighbors {
                neighbors_len += 1;

                // cohesion
                cohesion_point_sum += nbird.pos();

                // separation
                let opposed_force = bird.pos() - nbird.pos();
                let ivnorm = 1.0 / (norm!(opposed_force.x, opposed_force.y) as f32).powi(2);
                separation_force += self.separation * ivnorm * opposed_force;

                // match
                orientation_sum += nbird.orientation();
                velocity_sum += vec2(nbird.dx, nbird.dy);
            }

            if neighbors_len == 0 {
                continue;
            }

            let cohesion_avg_point = cohesion_point_sum / neighbors_len as f32;
            let cohesion_force = (cohesion_avg_point - bird.pos()) * self.cohesion;

            let velocity_avg = velocity_sum / neighbors_len as f32;
            let velocity_force = (velocity_avg - vec2(bird.dx, bird.dy)) * self.match_factor;

            let orientation_avg = orientation_sum / neighbors_len as f32;
            let alignment_force = self.match_factor
                * Vec2 {
                    x: orientation_avg.cos(),
                    y: orientation_avg.sin(),
                };

            const BORDER: f32 = 100.0;
            const BORDER_FORCE: f32 = 1.0;
            let (width, height) = self.window_size;
            let is_out_of_bound = bird.x <= BORDER
                || bird.x >= width as f32 - BORDER
                || bird.y <= BORDER
                || bird.y >= height as f32 - BORDER;
            if self.birds[bid].x <= BORDER {
                self.birds[bid].dx += BORDER_FORCE;
            }
            if self.birds[bid].x >= width as f32 - BORDER {
                self.birds[bid].dx -= BORDER_FORCE;
            }
            if self.birds[bid].y <= BORDER {
                self.birds[bid].dy += BORDER_FORCE;
            }
            if self.birds[bid].y >= height as f32 - BORDER {
                self.birds[bid].dy -= BORDER_FORCE;
            }

            if !is_out_of_bound {
                self.birds[bid].dx +=
                    cohesion_force.x + separation_force.x + alignment_force.x + velocity_force.x;
                self.birds[bid].dy +=
                    cohesion_force.y + separation_force.y + alignment_force.y + velocity_force.y;
            }

            self.birds[bid].dx = self.birds[bid].dx.min(5.0);
            self.birds[bid].dy = self.birds[bid].dy.min(5.0);

            self.birds[bid].x += self.birds[bid].dx;
            self.birds[bid].y += self.birds[bid].dy;
        }
    }
    pub fn render_birds(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        for bird in &self.birds {
            let bfv = bird.bird_frame_velocity();
            canvas.draw(
                &self.bird_frame[((self.bird_frame_number as usize) % (8 * bfv)) / bfv],
                graphics::DrawParam::new()
                    .dest(bird.pos())
                    // .scale(glam::Vec2::new(2.0, 2.0))
                    .rotation(bird.orientation()),
            );
        }
        Ok(())
    }
}

/*
dx = cos(orientation)
dy = sin(orientation)

dy/dx=tan(orientation)
*/
