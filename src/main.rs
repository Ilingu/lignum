mod app;
mod rendering;
mod utils;

use ggez::{conf::WindowMode, *};

use crate::app::State;

fn main() {
    let c = conf::Conf::new().window_mode(WindowMode::default().resizable(true));
    let init_window_size = c.window_mode;
    let (ctx, event_loop) = ContextBuilder::new("lignum", "ilingu")
        .default_conf(c)
        .build()
        .unwrap();

    let state = State::new(
        1000,
        &ctx.gfx,
        init_window_size.width,
        init_window_size.height,
    )
    .unwrap();
    event::run(ctx, event_loop, state);
}
