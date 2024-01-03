use skia_safe::{
    surfaces, Canvas, Color, FilterMode, ISize, Paint, Rect, SamplingOptions, Shader, TileMode,
    Vector,
};

fn main() {
    skia_safe_fiddle::init_window((6.0, 6.0), draw);
}

fn make() -> Option<Shader> {
    let mut surf = surfaces::raster_n32_premul(ISize::new(2, 2)).unwrap();
    surf.canvas().draw_color(Color::WHITE, None);
    surf.canvas()
        .draw_rect(Rect::new(0.0, 0.0, 1.0, 1.0), &Paint::default());
    surf.canvas()
        .draw_rect(Rect::new(1.0, 1.0, 2.0, 2.0), &Paint::default());
    surf.image_snapshot().to_shader(
        (TileMode::Repeat, TileMode::Repeat),
        SamplingOptions::from(FilterMode::Linear),
        None,
    )
}

fn draw(canvas: &Canvas) {
    canvas.draw_color(Color::WHITE, None);

    let r = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut p = Paint::default();
    p.set_shader(make());
    // this is a dither
    canvas.draw_rect(Rect::new(0.0, 0.0, 50.0, 50.0), &p);

    canvas.scale((0.5, 0.5));
    canvas.translate(Vector::new(100.0, 0.0));
    canvas.draw_rect(r, &p);
    p.set_shader(None);

    p.set_color(Color::new(0xff808080));
    canvas.translate(Vector::new(100.0, 0.0));
    canvas.draw_rect(r, &p);

    p.set_color(Color::new(0x80000000));
    canvas.translate(Vector::new(100.0, 0.0));
    canvas.draw_rect(r, &p);
}
