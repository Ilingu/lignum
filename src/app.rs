use ggegui::{egui, Gui};
use ggez::timer::TimeContext;
use ggez::winit::dpi::PhysicalSize;
use ggez::{
    glam::{vec2, Vec2},
    graphics::{self, Canvas, GraphicsContext, Image},
    Context, GameError,
};
use rayon::prelude::*;

use crate::norm;

const BORDER: f32 = 50.0;
const BORDER_COUCH: f32 = 0.1;

#[derive(Default, Clone, Copy)]
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
        const SPEED_MEAN: f32 = 0.1;
        (9.5 * (-SPEED_MEAN * (self.velocity().max(1.0) - 1.0)).exp() + 0.5)
            .min(10.0)
            .max(1.0)
            .ceil() as usize
    }

    pub fn velocity(&self) -> f32 {
        norm!(self.dx, self.dy) as f32
    }

    pub fn limit_velocity(&mut self, vlim: Option<f32>) {
        if let Some(vlim) = vlim {
            self.dx = self.dx.min(vlim).max(-vlim);
            self.dy = self.dy.min(vlim).max(-vlim);
            // if self.velocity().abs() > vlim {
            //     self.dx = (self.dx / self.dx.abs()) * vlim * FRAC_1_SQRT_2;
            //     self.dy = (self.dy / self.dy.abs()) * vlim * FRAC_1_SQRT_2;
            // }
        }
    }

    pub fn apply_forces(&mut self, forces: [Vec2; 4]) {
        self.dx += forces[0].x + forces[1].x + forces[2].x + forces[3].x;
        self.dy += forces[0].y + forces[1].y + forces[2].y + forces[3].y;
    }

    pub fn update_pos(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

pub struct State {
    // parameters
    pub separation: f32,
    pub match_factor: f32,
    pub cohesion: f32,
    pub vision_range: f32,
    pub vlim: Option<f32>,

    // assets
    pub bird_frame: Vec<Image>,

    // GUI
    pub gui: Gui,
    pub show_ui: bool,

    // game state
    pub birds: Vec<Bird>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            separation: 1.0,
            match_factor: 0.02,
            cohesion: 0.01,
            vision_range: 125.0,
            show_ui: true,
            vlim: Some(10.0),
            gui: Gui::default(),
            birds: Default::default(),
            bird_frame: Default::default(),
        }
    }
}

impl State {
    pub fn new(number_of_birds: usize, vlim: f32, ctx: &mut Context) -> Result<Self, GameError> {
        let mut rng = fastrand::Rng::new();

        // load bird frames
        let frames = (0..8)
            .map(|i| {
                graphics::Image::from_path(ctx, format!("/bird_frame/frame_{}.png", i)).unwrap()
            })
            .collect::<Vec<_>>();

        let PhysicalSize { width, height, .. } = ctx.gfx.window().inner_size();
        Ok(Self {
            bird_frame: frames,
            gui: Gui::new(ctx),
            vlim: if vlim == -1.0 { None } else { Some(vlim.abs()) },
            birds: (0..number_of_birds)
                .map(|id| Bird {
                    id,
                    x: rng.f32() * width as f32,
                    y: rng.f32() * height as f32,
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        })
    }

    pub fn compute_next_frame_parallel(&mut self, gfx: &GraphicsContext) {
        let PhysicalSize { width, height, .. } = gfx.window().inner_size();

        let mut new_birds = self.birds.clone();
        new_birds.par_iter_mut().for_each(|new_bird| {
            let bird = self.birds[new_bird.id];
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

            // Border repell
            if bird.x <= BORDER {
                let dst = BORDER - bird.x;
                new_bird.dx += dst * BORDER_COUCH;
            }
            if bird.x >= width as f32 - BORDER {
                let dst = bird.x - width as f32 + BORDER;
                new_bird.dx -= dst * BORDER_COUCH;
            }
            if bird.y <= BORDER {
                let dst = BORDER - bird.y;
                new_bird.dy += dst * BORDER_COUCH;
            }
            if bird.y >= height as f32 - BORDER {
                let dst = bird.y - height as f32 + BORDER;
                new_bird.dy -= dst * BORDER_COUCH;
            }

            // if no neighbors, return early to not corrupt the behavior of the bird by false datas
            if neighbors_len == 0 {
                new_bird.limit_velocity(self.vlim);
                new_bird.update_pos();
                return;
            }

            // compute forces
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

            new_bird.limit_velocity(self.vlim);
            new_bird.apply_forces([
                cohesion_force,
                separation_force,
                alignment_force,
                velocity_force,
            ]);
            new_bird.update_pos();
        });

        self.birds = new_birds;
    }

    pub fn render_birds(&self, canvas: &mut Canvas, time: &TimeContext) -> Result<(), GameError> {
        for bird in &self.birds {
            let bfv = bird.bird_frame_velocity();
            canvas.draw(
                &self.bird_frame[((time.ticks()) % (8 * bfv)) / bfv],
                graphics::DrawParam::new()
                    .dest(bird.pos())
                    .rotation(bird.orientation()),
            );
        }
        Ok(())
    }
    pub fn render_ui(&mut self, ctx: &mut Context) {
        if !self.show_ui {
            return;
        }

        let gui_ctx = self.gui.ctx();
        let window = egui::Window::new("Parameters")
            .resizable(false)
            .movable(false)
            .max_width(100.0);
        window.show(&gui_ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(format!("{} fps", ctx.time.fps().round()));
                ui.add(
                    egui::Slider::new(&mut self.cohesion, 0.0..=0.1)
                        .step_by(0.0001)
                        .text("Cohesion"),
                );
                ui.add(
                    egui::Slider::new(&mut self.separation, 0.0..=20.0)
                        .step_by(0.01)
                        .text("Separation"),
                );
                ui.add(
                    egui::Slider::new(&mut self.match_factor, 0.0..=0.5)
                        .step_by(0.001)
                        .text("Match"),
                );
                ui.add(
                    egui::Slider::new(&mut self.vision_range, 0.0..=200.0)
                        .step_by(1.0)
                        .text("Visual range"),
                );
            });
        });

        self.gui.update(ctx);
    }
}
