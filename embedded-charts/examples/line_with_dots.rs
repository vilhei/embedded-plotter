use embedded_charts::{
    axis::{Axis, Direction},
    line::LineChart,
};
use embedded_graphics::{
    geometry::OriginDimensions,
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor, Size},
    Drawable,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const POINT_COUNT: usize = 5;

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(160, 128));
    // let x_axis = Axis::default_x_axis();
    let x_axis = Axis::builder().color(Rgb565::BLUE).build();
    // let y_axis = Axis::builder()
    //     .direction(Direction::Vertical)
    //     .max(36)
    //     .build();

    let mut line_plot: LineChart<_, POINT_COUNT> = LineChart::builder()
        // .points(test_points)
        .line_color(Rgb565::WHITE)
        .point_color(Rgb565::RED)
        .line_width(2)
        .y_max(36)
        .x_axis(x_axis)
        .build();

    for i in 1..POINT_COUNT + 1 {
        line_plot.push(Point {
            x: i as i32 * 10,
            y: (i as i32 + 1).pow(2),
        })
    }

    line_plot.scale_points_to_display(&display.size());
    line_plot.draw(&mut display).unwrap();
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("Line with dots", &output_settings).show_static(&display);
}
