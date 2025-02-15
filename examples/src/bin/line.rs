#![no_std]
#![no_main]

// use embedded_charts::line::LineChart;
use esp_backtrace as _;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    delay::Delay,
    esp_riscv_rt::entry,
    gpio::{interconnect::PeripheralOutput, Level, Output, OutputPin},
    peripheral::Peripheral,
    spi::{
        master::{Instance, Spi},
        SpiMode,
    },
    Blocking,
};
use esp_println::println;
use fugit::RateExtU32;
use mipidsi::{
    interface::SpiInterface,
    models::ST7735s,
    options::{Orientation, Rotation},
};
use static_cell::ConstStaticCell;

static DISPLAY0_BUFFER: ConstStaticCell<[u8; 512]> = ConstStaticCell::new([0u8; 512]);

const POINT_COUNT: usize = 5;

#[entry]
fn main() -> ! {
    let per = esp_hal::init(Default::default());

    let test_points: [Point; POINT_COUNT] = [
        Point::new(5, 5),
        Point::new(13, 5),
        Point::new(20, 10),
        Point::new(40, 20),
        Point::new(100, 90),
    ];

    let mut line_plot: LineChart<_, POINT_COUNT> = LineChart::builder()
        // .points(test_points)
        .line_color(Rgb565::WHITE)
        .point_color(Rgb565::RED)
        .line_width(2)
        .y_max(36)
        .x_max(100)
        .build();

    for i in 1..POINT_COUNT + 1 {
        line_plot.push(Point {
            x: i as i32 * 10,
            y: (i as i32 + 1).pow(2),
        })
    }
    println!("{:?}", line_plot.get_points());

    let mut display = setup_display(
        per.SPI2, per.GPIO3, per.GPIO4, per.GPIO0, per.GPIO2, per.GPIO1,
    );

    display.clear(Rgb565::BLACK).unwrap();
    println!("{:?}", display.size());
    line_plot.scale_points_to_display(&display.size());

    println!("{:?}", line_plot.get_points());
    line_plot.draw(&mut display).unwrap();
    println!("Done");

    loop {
        esp_hal::riscv::asm::wfi();
    }
}

fn setup_display(
    spi: impl Peripheral<P = impl Instance> + 'static,
    mosi: impl Peripheral<P = impl PeripheralOutput> + 'static,
    sck: impl Peripheral<P = impl PeripheralOutput> + 'static,
    cs: impl Peripheral<P = impl OutputPin> + 'static,
    reset: impl Peripheral<P = impl OutputPin> + 'static,
    dc: impl Peripheral<P = impl OutputPin> + 'static,
) -> mipidsi::Display<
    SpiInterface<
        'static,
        ExclusiveDevice<esp_hal::spi::master::Spi<'static, Blocking>, Output<'static>, Delay>,
        Output<'static>,
    >,
    ST7735s,
    Output<'static>,
> {
    let spi = Spi::new_with_config(
        spi,
        esp_hal::spi::master::Config {
            frequency: 40u32.MHz(),
            mode: SpiMode::Mode0,
            ..Default::default()
        },
    )
    .with_mosi(mosi)
    .with_sck(sck);

    let cs = Output::new(cs, Level::High);

    let mut delay = Delay::new();

    let mut reset = Output::new(reset, Level::Low);
    reset.set_high();
    let dc = Output::new(dc, Level::Low);

    let spi_display = ExclusiveDevice::new(spi, cs, delay).unwrap();
    let display_interface = SpiInterface::new(spi_display, dc, DISPLAY0_BUFFER.take());

    mipidsi::Builder::new(ST7735s, display_interface)
        .reset_pin(reset)
        .display_size(128, 160)
        .orientation(Orientation::new().rotate(Rotation::Deg270))
        .init(&mut delay)
        .unwrap()
}
