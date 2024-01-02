use skia_safe::Canvas;
fn main() {
    skia_safe_fiddle::init_window(draw);
}
fn draw(canvas: &Canvas) {
    use skia_safe::{Color, Paint, Point};

    // Code Start:
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    canvas.draw_circle(Point::new(128.0, 128.0), 90.0, &paint);
    paint.set_color(Color::WHITE);
    canvas.draw_circle(Point::new(86.0, 86.0), 20.0, &paint);
    canvas.draw_circle(Point::new(160.0, 76.0), 20.0, &paint);
    canvas.draw_circle(Point::new(140.0, 150.0), 35.0, &paint);
}
