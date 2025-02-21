#![allow(dead_code)]
use bon::builder;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::prelude::{Dimensions, PixelColor, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle};
use embedded_graphics::Drawable;

use crate::axis::Axis;
use crate::scale_point;

// TODO should the Line own the data (points) or should it take reference?
// This leads to lifetime questions

#[derive(Debug, bon::Builder)]
pub struct LineChart<'a, C, const SAMPLES: usize = 5>
where
    C: Default,
{
    #[builder(default = [None; SAMPLES], with = |points:[Point;SAMPLES]| points.map(|p| Some(p)))]
    points: [Option<Point>; SAMPLES],
    #[builder(default = Axis::default_x_axis())]
    x_axis: Axis<'a, C>,
    #[builder(default = Axis::default_y_axis())]
    y_axis: Axis<'a, C>,
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

impl<C, const SAMPLES: usize> LineChart<'_, C, SAMPLES>
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

    /// Scale points to display coordinates. Should be called before drawing.
    /// Or optionally scale the data before inserting outside of this struct
    pub fn scale_points_to_display<D: Dimensions>(&mut self, display: &D) {
        self.points = self.points.map(|point| match point {
            Some(p) => Some(scale_point(
                p,
                &display.bounding_box().size,
                &self.x_axis.calculate_starting_coordinates(display),
                // &self.x_axis.origin_offset,
                self.x_axis.min,
                self.x_axis.max,
                self.y_axis.min,
                self.y_axis.max,
            )),
            None => None,
        });
    }
}

impl<C: Default> Default for LineChart<'_, C> {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl<C, const SAMPLES: usize> Drawable for LineChart<'_, C, SAMPLES>
where
    C: PixelColor + Default,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.x_axis.draw(target)?;
        self.y_axis.draw(target)?;
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
