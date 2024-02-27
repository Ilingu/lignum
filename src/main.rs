mod app;
mod rendering;
mod utils;

use std::path;

use clap::Parser;
use ggez::{conf::WindowMode, *};

use crate::app::State;

/// Simple implementation for boids algorithm
#[derive(Parser, Debug)]
#[command(name = "Lignum")]
#[command(author = "Ilingu")]
#[command(version = "0.1")]
#[command(about = "Simple implementation for boids algorithm, but now it's more a fluid simulation game~", long_about = None)]
#[command(propagate_version = true)]
struct LignumArgs {
    #[arg(long, default_value_t = 1000, verbatim_doc_comment)]
    /// Initial number of bird in the sky~
    /// Since my program is not well optimized, >5000 birds will be very laggy
    bird_count: usize,

    /// If true: set to 4 samples, otherwise (default) set to 1 sample
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    strong_anti_aliasing: bool,

    /// Limit velocity for a bird (default 10), -1.0 to disable
    #[arg(long, default_value_t = 10.0, verbatim_doc_comment)]
    vlim: f32,
}

fn main() {
    let cli_args = LignumArgs::parse();

    let resource_dir = path::PathBuf::from("./resources");
    let winconf = conf::Conf::new().window_mode(WindowMode::default().resizable(true));
    let (mut ctx, event_loop) = ContextBuilder::new("lignum", "ilingu")
        .default_conf(winconf)
        .add_resource_path(resource_dir)
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Lignum")
                .icon("/bird_frame/frame_0.png")
                .samples(if cli_args.strong_anti_aliasing {
                    conf::NumSamples::Four
                } else {
                    conf::NumSamples::One
                }),
        )
        .build()
        .unwrap();

    let state = State::new(cli_args.bird_count, cli_args.vlim, &mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}
