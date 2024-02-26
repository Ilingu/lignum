use ggez::{graphics::Color, mint::Point2, *};

use crate::State;

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.bird_frame_number = self.bird_frame_number.wrapping_add(1);
        self.update_window_size(&ctx.gfx);
        self.compute_next_frame();

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(40, 44, 52));
        self.render_birds(&mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn build_textured_triangle(
    ctx: &mut Context,
    Point2 { x, y }: &Point2<f64>,
    color: Color,
    size: f64,
    angles: [f64; 3],
) -> graphics::Mesh {
    let (r, g, b) = color.to_rgb();
    let percent_rgb = [r, g, b, 255].map(|x| ((x as f64) / 255.0) as f32);

    let triangle_verts = vec![
        graphics::Vertex {
            position: [
                (x + size * angles[0].cos()) as f32,
                (y + size * angles[0].sin()) as f32,
            ],
            uv: [1.0, 1.0],
            color: percent_rgb,
        },
        graphics::Vertex {
            position: [
                (x + size * angles[1].cos()) as f32,
                (y + size * angles[1].sin()) as f32,
            ],
            uv: [0.0, 1.0],
            color: percent_rgb,
        },
        graphics::Vertex {
            position: [
                (x + size * angles[2].cos()) as f32,
                (y + size * angles[2].sin()) as f32,
            ],
            uv: [0.0, 0.0],
            color: percent_rgb,
        },
    ];

    let triangle_indices = vec![0, 1, 2];

    graphics::Mesh::from_data(
        ctx,
        graphics::MeshData {
            vertices: &triangle_verts,
            indices: &triangle_indices,
        },
    )
}
