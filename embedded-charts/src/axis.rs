use core::default::Default;

use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point, Primitive},
    primitives::{Line, PrimitiveStyle},
    Drawable,
};

#[derive(Debug, bon::Builder)]
pub struct Axis<C>
where
    C: Default,
{
    #[builder(default = 100)]
    max: i32,
    #[builder(default = 0)]
    min: i32,
    #[builder(default=Direction::Horizontal)]
    direction: Direction,
    #[builder(default = true)]
    visible: bool,
    #[builder(default)]
    color: C,
    #[builder(default = 2)]
    line_width: u32,
    // TODO add possibility to change top/bottom and left/right position for axis
}

impl<C> Axis<C>
where
    C: Default,
{
    pub fn default_x_axis() -> Self {
        Axis::builder().build()
    }

    pub fn default_y_axis() -> Self {
        Axis::builder().direction(Direction::Vertical).build()
    }

    pub fn min(&self) -> i32 {
        self.min
    }
    pub fn max(&self) -> i32 {
        self.max
    }
}

impl<C> Drawable for Axis<C>
where
    C: PixelColor + Default,
{
    type Color = C;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        if !self.visible {
            return Ok(());
        }

        let height = target.bounding_box().size.height;
        let width = target.bounding_box().size.width;

        match self.direction {
            Direction::Horizontal => {
                let line_height = height as i32 - 4;
                let start_point = Point::new(0, line_height);
                let end_point = Point::new(width as i32, line_height);

                Line::new(start_point, end_point)
                    .into_styled(PrimitiveStyle::with_stroke(self.color, self.line_width))
                    .draw(target)?;
            }
            Direction::Vertical => todo!(),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}
