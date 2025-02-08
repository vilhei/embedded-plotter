#![allow(dead_code)]
use bon::builder;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{PixelColor, Point, Primitive};
use embedded_graphics::primitives::{Arc, Circle, Line, PrimitiveStyle};
use embedded_graphics::Drawable;

// TODO should the Line own the data (points) or should it take reference?
// This leads to lifetime questions

#[derive(Debug, bon::Builder)]
pub struct LineChart<C, const SAMPLES: usize = 5>
where
    C: Default,
{
    #[builder(default = [None; SAMPLES], with = |points:[Point;SAMPLES]| points.map(|p| Some(p)))]
    points: [Option<Point>; SAMPLES],
    #[builder(default = SAMPLES as i32)]
    x_max: i32,
    #[builder(default)]
    x_min: i32,
    #[builder(default = 100)]
    y_max: i32,
    #[builder(default)]
    y_min: i32,
    #[builder(default)]
    line_color: C,
    #[builder(default)]
    point_color: C,
    #[builder(default = 3)]
    line_width: u32,
    #[builder(default = 5)]
    point_diameter: u32,
    #[builder(default = true)]
    show_points: bool,
}

impl<C, const SAMPLES: usize> LineChart<C, SAMPLES>
where
    C: Default,
{
    pub fn push(&mut self, new_point: Point) {
        self.points.rotate_right(1);
        self.points[0] = Some(new_point);
    }
    pub fn get_points(&self) -> &[Option<Point>] {
        &self.points
    }
}

impl<C: Default> Default for LineChart<C> {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl<C, const SAMPLES: usize> Drawable for LineChart<C, SAMPLES>
where
    C: PixelColor + Default,
{
    type Color = C;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let width = target.bounding_box().size.width;
        let height = target.bounding_box().size.height;
        for w in self.points.windows(2) {
            if let (Some(mut p1), Some(mut p2)) = (w[0], w[1]) {
                p1 = Point {
                    y: height as i32 - p1.y,
                    ..p1
                };
                p2 = Point {
                    y: height as i32 - p2.y,
                    ..p2
                };
                if self.show_points {
                    Circle::with_center(p1, self.point_diameter)
                        .into_styled(PrimitiveStyle::with_fill(self.point_color))
                        .draw(target)?;
                    // TODO figure out better way to draw the last point instead of drawing other points 2 times
                    Circle::with_center(p2, self.point_diameter)
                        .into_styled(PrimitiveStyle::with_fill(self.point_color))
                        .draw(target)?;
                }

                Line::new(p1, p2)
                    .into_styled(PrimitiveStyle::with_stroke(
                        self.line_color,
                        self.line_width,
                    ))
                    .draw(target)?;
            } else {
                break;
            }
        }
        Ok(())
    }
}
