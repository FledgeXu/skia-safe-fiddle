use skia_safe::{Canvas, Paint, PaintStyle, Point, Vector};

fn main() {
    skia_safe_fiddle::init_window((6.0, 6.0), draw);
}

fn draw(canvas: &Canvas) {
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    canvas.translate(Vector::new(128.0, 128.0));
    canvas.draw_circle(Point::new(0.0, 0.0), 60.0, &paint);
    canvas.save();
    canvas.rotate(10.0 * 360.0 / 60.0, None); // 10 minutes of 60 scaled to 360 degrees
    canvas.draw_line(Point::new(0.0, 0.0), Point::new(0.0, -50.0), &paint);
    canvas.restore();
    canvas.rotate((5.0 + 10.0 / 60.0) * 360.0 / 12.0, None); // 5 and 10/60 hours of 12 scaled to 360 degrees
    canvas.draw_line(Point::new(0.0, 0.0), Point::new(0.0, -30.0), &paint);
}
