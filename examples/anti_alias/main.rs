use skia_safe::{Bitmap, Canvas, Color, Paint, PaintStyle};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn draw(canvas: &Canvas) {
    let mut bitmap = Bitmap::default();
    bitmap.alloc_n32_pixels((50, 50), true);
    let offscreen = Canvas::from_bitmap(&bitmap, None).unwrap();
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(10.0);
    for antialias in vec![false, true] {
        paint.set_color(if antialias { Color::RED } else { Color::BLUE });
        paint.set_anti_alias(antialias);
        bitmap.erase_color(Color::TRANSPARENT);
        offscreen.draw_line((5.0, 5.0), (15.0, 30.0), &paint);
        canvas.draw_line((5.0, 5.0), (15.0, 30.0), &paint);
        canvas.save();
        canvas.scale((10.0, 10.0));
        canvas.draw_image(
            &bitmap.as_image(),
            if antialias { (12.0, 0.0) } else { (0.0, 0.0) },
            None,
        );
        canvas.restore();
        canvas.translate((15.0, 0.0));
    }
}
