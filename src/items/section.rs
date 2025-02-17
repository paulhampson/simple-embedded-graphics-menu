use crate::items::{DrawableHighlighted, MenuItem, MenuItemData, SelectedData};
use crate::MenuStyle;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(PartialEq, Clone, Copy)]
pub struct SectionItem<'a, C, T>
where
    C: PixelColor,
{
    label: &'static str,
    highlighted: bool,
    position: Point,
    menu_style: MenuStyle<'a, C>,
    id: T,
}

impl<C, T> SectionItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    pub const fn new<'a>(
        label: &'static str,
        id: T,
        menu_style: MenuStyle<'a, C>,
    ) -> SectionItem<'a, C, T> {
        SectionItem {
            label,
            highlighted: false,
            position: Point::zero(),
            menu_style,
            id,
        }
    }
}

impl<C, T> MenuItem<T> for SectionItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    fn label(&self) -> &'static str {
        self.label
    }
    fn id(&self) -> T {
        self.id
    }
}

impl<C: PixelColor, T> Debug for SectionItem<'_, C, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":Section]", self.label)
    }
}

impl<C: PixelColor, T> Display for SectionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<C: PixelColor, T> View for SectionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
    }

    fn bounds(&self) -> Rectangle {
        self.menu_style
            .item_character_style
            .measure_string(self.label, Point::zero(), Baseline::Bottom)
            .bounding_box
    }
}

impl<C: PixelColor, T> Drawable for SectionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Text::with_text_style(
            self.label,
            Point::new(display.bounding_box().size().width as i32 / 2, 0),
            self.menu_style.item_character_style,
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}

impl<C: PixelColor, T> DrawableHighlighted for SectionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    type Color = C;
    type Output = ();

    fn draw_highlighted<D>(&self, _display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Ok(())
    }
}

impl<C, T> MenuItemData<T> for SectionItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    fn selected(&mut self) -> SelectedData<T> {
        SelectedData::Section { id: self.id }
    }

    fn display_string(&self) -> &str {
        self.label()
    }
}
