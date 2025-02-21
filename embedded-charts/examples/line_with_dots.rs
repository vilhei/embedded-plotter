use embedded_charts::{
    axis::{calibrate_starting_points, Axis, Direction},
    line::LineChart,
};
use embedded_graphics::{
    geometry::OriginDimensions,
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor, Size},
    Drawable,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const POINT_COUNT: usize = 1000;

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(160, 128));
    // let x_axis = Axis::default_x_axis();
    let axis_origin = Point {
        x: 4,
        y: display.size().height as i32 - 4,
    };

    let mut x_axis = Axis::builder()
        .max(POINT_COUNT as i32)
        .color(Rgb565::BLUE)
        .line_width(1)
        .show_legend(true)
        .legend("x")
        .build();
    let mut y_axis = Axis::builder()
        .direction(Direction::Vertical)
        .max(36)
        .line_width(1)
        .color(Rgb565::RED)
        .show_legend(true)
        .legend("y")
        .build();
    println!("{:?})", x_axis);
    println!("{:?})", y_axis);
    calibrate_starting_points(&mut x_axis, &mut y_axis, &display).unwrap();
    println!("{:?})", x_axis);
    println!("{:?})", y_axis);

    let mut line_plot: LineChart<_, POINT_COUNT> = LineChart::builder()
        .line_color(Rgb565::WHITE)
        .point_color(Rgb565::RED)
        .line_width(2)
        .y_max(36)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .show_points(false)
        .build();

    for x in 0..POINT_COUNT {
        line_plot.push(Point {
            x: x as i32,
            y: ((x as f32 * 0.01).sin() * 10.0 + 20.0) as i32,
        })
    }
    // println!("{:?}", line_plot.get_points());

    line_plot.scale_points_to_display(&display.size());
    // println!("{:?}", line_plot.get_points());
    line_plot.draw(&mut display).unwrap();
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("Line with dots", &output_settings).show_static(&display);
}
