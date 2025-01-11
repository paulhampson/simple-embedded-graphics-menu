use core::fmt;
use core::fmt::Formatter;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuItemType {
    Section,
    Checkbox,
    Selector,
    Submenu,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MenuItem<'a, C>
where
    C: PixelColor,
{
    label: &'static str,
    item_type: MenuItemType,
    highlighted: bool,
    character_style: MonoTextStyle<'a, C>,
    position: Point,
}

impl<C> MenuItem<'_, C>
where
    C: PixelColor,
{
    pub const fn new<'a>(
        label: &'static str,
        item_type: MenuItemType,
        character_style: MonoTextStyle<'a, C>,
    ) -> MenuItem<'a, C> {
        MenuItem::<'a, C> {
            label,
            item_type,
            highlighted: false,
            character_style,
            position: Point::zero(),
        }
    }

    pub fn label(&self) -> &str {
        self.label
    }
}

impl<C> fmt::Display for MenuItem<'_, C>
where
    C: PixelColor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":{:?}]", self.label, self.item_type)
    }
}

impl<C> View for MenuItem<'_, C>
where
    C: PixelColor,
{
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
    }

    fn bounds(&self) -> Rectangle {
        self.character_style
            .measure_string(self.label, Point::zero(), Baseline::Bottom)
            .bounding_box
    }
}

impl<C> Drawable for MenuItem<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let item_text = Text::with_baseline(
            self.label,
            self.position,
            self.character_style,
            Baseline::Top,
        );
        item_text.draw(display)?;
        Ok(())
    }
}
