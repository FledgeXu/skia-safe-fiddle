use skia_safe::{Canvas, Color, Paint};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn draw(canvas: &Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    canvas.draw_circle((128.0, 128.0), 90.0, &paint);
    paint.set_color(Color::WHITE);
    canvas.draw_circle((86.0, 86.0), 20.0, &paint);
    canvas.draw_circle((160.0, 76.0), 20.0, &paint);
    canvas.draw_circle((140.0, 150.0), 35.0, &paint);
}
