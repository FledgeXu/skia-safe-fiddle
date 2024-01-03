use skia_safe::{Canvas, Color, Rect};

fn main() {
    skia_safe_fiddle::init_window((5.0, 5.0), draw);
}

fn draw(canvas: &Canvas) {
    canvas.save();
    canvas.clip_rect(Rect::from_wh(256.0, 128.0), None, None);
    canvas.clear(Color::from_argb(0x80, 0xff, 0x00, 0x00));
    canvas.restore();
    canvas.save();
    canvas.clip_rect(Rect::from_wh(150.0, 192.0), None, None);
    canvas.clear(Color::from_argb(0x80, 0x00, 0xFF, 0x00));
    canvas.restore();
    canvas.clip_rect(Rect::from_wh(75.0, 256.0), None, None);
    canvas.clear(Color::from_argb(0x80, 0x00, 0x00, 0xFF));
}
