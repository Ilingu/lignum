use ggez::{
    graphics::DrawParam,
    input::keyboard::{KeyCode, KeyInput},
    *,
};

use crate::State;

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.compute_next_frame_parallel(&ctx.gfx);
        self.render_ui(ctx);

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(40, 44, 52));

        self.render_birds(&mut canvas, &ctx.time)?;
        canvas.draw(&self.gui, DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
    /// `key_down_event` gets fired when a key gets pressed.
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
        if repeat {
            return Ok(());
        }
        if let Some(KeyCode::U) = input.keycode {
            self.show_ui = !self.show_ui;
        }
        Ok(())
    }
}
