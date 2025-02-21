use core::default::Default;

use axis_builder::{SetLegend, State};
use embedded_graphics::{
    geometry::AnchorPoint,
    prelude::{Dimensions, DrawTarget, PixelColor, Point, Primitive},
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
    Drawable,
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

/// Default origin is in 0,0 which probably does not work for you.
#[derive(Debug, bon::Builder)]
pub struct Axis<'a, C>
where
    C: Default,
{
    #[builder(default = 100)]
    pub max: i32,
    #[builder(default = 0)]
    pub min: i32,
    // TODO move Direction to be generic type?
    #[builder(default=Direction::Horizontal)]
    direction: Direction,
    #[builder(default = true)]
    visible: bool,
    #[builder(default)]
    color: C,
    #[builder(default = 2)]
    line_width: u32,
    #[builder(default = [3,3], into, setters(vis = ""))]
    /// In pixel coordinates of the display
    starting_point_offset: Point,
    #[builder(default)]
    show_legend: bool,
    #[builder(default = "x")]
    legend: &'a str,
    // TODO add possibility to change top/bottom and left/right position for axis
}

impl<C> Axis<'_, C>
where
    C: Default,
{
    pub fn calculate_starting_coordinates<D: Dimensions>(&self, display: &D) -> Point {
        let bounding_box = display.bounding_box();
        let height = bounding_box.size.height;
        let mut start_point = self.starting_point_offset;
        start_point.y = height as i32 - start_point.y;

        const LEGEND_PADDING: i32 = 0;
        match self.direction {
            Direction::Horizontal => {
                if self.show_legend {
                    let legend_result = crate::DEFAULT_FONT.get_rendered_dimensions_aligned(
                        self.legend,
                        bounding_box.anchor_point(AnchorPoint::BottomCenter),
                        VerticalPosition::Bottom,
                        HorizontalAlignment::Center,
                    );
                    match legend_result {
                        Ok(Some(r)) => {
                            // start_point.x += r.top_left.x;
                            start_point.y -= r.top_left.y.abs_diff(start_point.y) as i32
                                + self.line_width as i32
                                + 1;
                        }
                        Ok(_) => (),
                        Err(_) => todo!(),
                    }
                }
            }
            Direction::Vertical => {
                if self.show_legend {
                    let legend_result = crate::DEFAULT_FONT.get_rendered_dimensions_aligned(
                        self.legend,
                        bounding_box.anchor_point(AnchorPoint::CenterLeft),
                        VerticalPosition::Center,
                        HorizontalAlignment::Left,
                    );
                    match legend_result {
                        Ok(Some(r)) => {
                            start_point.x += r.size.width as i32;
                        }
                        Ok(_) => (),
                        Err(_) => todo!(),
                    }
                }
            }
        }
        start_point
    }

    pub fn update_starting_offset(&mut self, new_offset: Point) {
        self.starting_point_offset = new_offset;
    }

    pub fn default_x_axis() -> Self {
        Axis::builder().build()
    }

    pub fn default_y_axis() -> Self {
        Axis::builder().direction(Direction::Vertical).build()
    }
}

impl<C> Axis<'_, C>
where
    C: PixelColor + Default,
{
    // TODO combine x and y triangle to single function
    fn x_triangle(
        &self,
        end_point: &Point,
    ) -> embedded_graphics::primitives::Styled<Triangle, PrimitiveStyle<C>> {
        let triangle_point1 = Point {
            x: end_point.x,
            y: end_point.y + 2,
        };
        let triangle_point2 = Point {
            x: end_point.x,
            y: end_point.y - 3,
        };
        let triangle_point3 = Point {
            x: end_point.x + 5,
            y: end_point.y,
        };
        let triangle = Triangle::new(triangle_point1, triangle_point2, triangle_point3)
            .into_styled(PrimitiveStyle::with_fill(self.color));
        triangle
    }

    fn y_triangle(
        &self,
        end_point: Point,
    ) -> embedded_graphics::primitives::Styled<Triangle, PrimitiveStyle<C>> {
        let triangle_point1 = Point {
            x: end_point.x + 2,
            y: end_point.y,
        };
        let triangle_point2 = Point {
            x: end_point.x - 3,
            y: end_point.y,
        };
        let triangle_point3 = Point {
            x: end_point.x,
            y: end_point.y - 5,
        };
        let triangle = Triangle::new(triangle_point1, triangle_point2, triangle_point3)
            .into_styled(PrimitiveStyle::with_fill(self.color));
        triangle
    }
}

impl<C> Drawable for Axis<'_, C>
where
    C: PixelColor + Default,
{
    type Color = C;

    // TODO return the space taken by axis? to allow the plots
    // be correctly scaled "inside" the axis
    type Output = Point;

    // TODO change origin based on if the legend is drawn or not
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let end_padding = 8; // enough for triangle

        if !self.visible {
            todo!();
            // return Ok(());
        }

        let width = target.bounding_box().size.width;
        let height = target.bounding_box().size.height;
        let mut start_point = self.starting_point_offset;
        start_point.y = height as i32 - start_point.y;

        const LEGEND_PADDING: i32 = 0;
        match self.direction {
            Direction::Horizontal => {
                if self.show_legend {
                    let legend_result = crate::DEFAULT_FONT.render_aligned(
                        self.legend,
                        target
                            .bounding_box()
                            .anchor_point(AnchorPoint::BottomCenter),
                        VerticalPosition::Bottom,
                        HorizontalAlignment::Center,
                        FontColor::Transparent(self.color),
                        target,
                    );
                    match legend_result {
                        Ok(Some(r)) => {
                            // start_point.x += r.top_left.x;
                            // TODO check this, probably not correct
                            start_point.y -= r.top_left.y.abs_diff(start_point.y) as i32
                                + self.line_width as i32
                                + 1;
                        }
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                }
                let end_point = Point::new(width as i32 - end_padding, start_point.y);
                Line::new(start_point, end_point)
                    .into_styled(PrimitiveStyle::with_stroke(self.color, self.line_width))
                    .draw(target)?;

                let triangle = self.x_triangle(&end_point);
                triangle.draw(target)?;
            }
            Direction::Vertical => {
                if self.show_legend {
                    let legend_result = crate::DEFAULT_FONT.render_aligned(
                        self.legend,
                        target.bounding_box().anchor_point(AnchorPoint::CenterLeft),
                        VerticalPosition::Center,
                        HorizontalAlignment::Left,
                        FontColor::Transparent(self.color),
                        target,
                    );
                    match legend_result {
                        Ok(Some(r)) => {
                            start_point.x += r.size.width as i32;
                        }
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                }
                let end_point = Point::new(start_point.x, end_padding);

                Line::new(start_point, end_point)
                    .into_styled(PrimitiveStyle::with_stroke(self.color, self.line_width))
                    .draw(target)?;

                let triangle = self.y_triangle(end_point);
                triangle.draw(target)?;
            }
        }
        Ok(start_point)
    }
}

#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Helper type for having one vertical and one horizontal axis with same starting point
pub struct AxisPair {}

pub fn calibrate_starting_points<C1, C2, D>(
    horizontal: &mut Axis<C1>,
    vertical: &mut Axis<C2>,
    display: &D,
) -> Result<(), ()>
where
    C1: Default,
    C2: Default,
    D: Dimensions,
{
    match horizontal.direction {
        Direction::Horizontal => (),
        Direction::Vertical => return Err(()),
    }
    match vertical.direction {
        Direction::Horizontal => return Err(()),
        Direction::Vertical => (),
    }

    let hor_s_point = horizontal.calculate_starting_coordinates(display);
    let ver_s_point = vertical.calculate_starting_coordinates(display);

    if ver_s_point.y != hor_s_point.y {
        vertical.starting_point_offset.y += ver_s_point.y.abs_diff(hor_s_point.y) as i32;
    }

    if ver_s_point.x != hor_s_point.x {
        horizontal.starting_point_offset.x += ver_s_point.x.abs_diff(hor_s_point.x) as i32;
    }

    Ok(())
}
