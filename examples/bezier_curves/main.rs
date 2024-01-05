use skia_safe::{Canvas, Color, Paint, PaintCap, PaintStyle, Path};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn draw(canvas: &Canvas) {
    canvas.draw_color(Color::WHITE, None);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(8.0);
    paint.set_color(0xFF4285F4);
    paint.set_anti_alias(true);
    paint.set_stroke_cap(PaintCap::Square);

    let mut path = Path::default();
    path.move_to((10.0, 10.0));
    path.quad_to((256.0, 64.0), (128, 128));
    path.quad_to((10.0, 192.0), (250, 250));
    canvas.draw_path(&path, &paint);
}
