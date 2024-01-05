use skia_safe::{
    gradient_shader::{self, Flags, GradientShaderColors},
    Canvas, Color, Paint, TileMode,
};

fn main() {
    skia_safe_fiddle::init_window((3.0, 3.0), draw);
}

fn draw(canvas: &Canvas) {
    let points = ((0.0, 0.0), (256.0, 256.0));
    let colors = vec![Color::BLUE, Color::YELLOW];
    let mut paint = Paint::default();
    paint.set_shader(gradient_shader::linear(
        points,
        GradientShaderColors::Colors(&colors),
        None,
        TileMode::Clamp,
        Flags::from_bits(0),
        None,
    ));
    canvas.draw_paint(&paint);
}
