use skia_safe::{
    surfaces, Canvas, Color, FilterMode, Font, ISize, Paint, Rect, SamplingOptions, Shader,
    TileMode, Typeface,
};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn make_bw_dither() -> Option<Shader> {
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
    let font = Font::new(Typeface::default(), 12.0);

    // BW Dither
    canvas.translate((5.0, 5.0));
    let mut p = Paint::default();
    p.set_shader(make_bw_dither());
    canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), &p);
    let balck = Paint::default();
    canvas.draw_str("BW Dither", (0.0, 125.0), &font, &balck);

    // Scaled BW Dither
    canvas.translate((105.0, 0.0));
    canvas.save();
    canvas.scale((0.5, 0.5));
    canvas.draw_rect(Rect::new(0.0, 0.0, 200.0, 200.0), &p);
    canvas.restore();
    canvas.draw_str("Scaled Dither", (0.0, 125.0), &font, &balck);

    // Blend black on to white
    canvas.translate((105.0, 0.0));
    p.set_color(Color::new(0x80000000));
    p.set_shader(None);
    canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), &p);
    canvas.draw_str("Blend", (0.0, 125.0), &font, &balck);

    // Opaque color (0xFFBCBCBC)
    canvas.translate((105.0, 0.0));
    p.set_color(Color::new(0xFFBCBCBC));
    canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), &p);
    canvas.draw_str("0xFFBCBCBC", (0.0, 125.0), &font, &balck);

    // Opaque color (0xFF808080)
    canvas.translate((105.0, 0.0));
    p.set_color(Color::new(0xFF808080));
    canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), &p);
    canvas.draw_str("0xFF808080", (0.0, 125.0), &font, &balck);
}
