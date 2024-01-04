use skia_safe::{Canvas, Color, Paint, PaintStyle, Point, RRect, Rect, Vector};

fn main() {
    skia_safe_fiddle::init_window((5.0, 5.0), draw);
}

fn draw(canvas: &Canvas) {
    canvas.draw_color(Color::WHITE, None);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_anti_alias(true);
    paint.set_stroke_width(4.0);
    paint.set_color(Color::new(0xFF4285F4));

    let mut rect = Rect::from_xywh(10.0, 10.0, 100.0, 160.0);
    canvas.draw_rect(&rect, &paint);

    let mut oval = RRect::default();
    oval.set_oval(rect);
    oval.offset(Vector::new(40.0, 80.0));
    paint.set_color(0xFFDB4437);
    canvas.draw_rrect(&oval, &paint);

    paint.set_color(0xFF0F9D58);
    canvas.draw_circle(Point::new(180.0, 50.0), 25.0, &paint);

    rect.offset(Vector::new(80.0, 50.0));
    paint.set_color(0xFFF4B400);
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_round_rect(rect, 10.0, 10.0, &paint);
}
