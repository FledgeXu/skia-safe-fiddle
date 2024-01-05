use skia_safe::{Canvas, Color, Paint, PaintStyle, Point};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn draw(canvas: &Canvas) {
    let mut p = Paint::default();
    p.set_color(Color::RED);
    p.set_anti_alias(true);
    p.set_style(PaintStyle::Stroke);
    p.set_stroke_width(10.0);

    canvas.draw_line(Point::new(20.0, 20.0), Point::new(100.0, 100.0), &p);
}
