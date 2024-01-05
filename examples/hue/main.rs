use skia_safe::{BlendMode, Canvas, Point};
use skia_safe_fiddle::resources;

fn main() {
    skia_safe_fiddle::init_window((5.0, 5.0), draw);
}

fn draw(canvas: &Canvas) {
    let image = resources::example3();
    canvas.draw_image(&image, Point::new(0.0, 0.0), None);
    canvas.draw_color(0xFF00FF00, BlendMode::Hue);
}
