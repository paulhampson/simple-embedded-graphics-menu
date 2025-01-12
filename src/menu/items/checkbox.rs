use crate::menu::items::{MenuItem, MenuItemData, MenuItemWithData};
use crate::menu::MenuStyle;
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
pub struct CheckboxItem<'a, C>
where
    C: PixelColor,
{
    label: &'static str,
    highlighted: bool,
    position: Point,
    menu_style: MenuStyle<'a, C>,
    checkbox_state: bool,
}

impl<C> CheckboxItem<'_, C>
where
    C: PixelColor,
{
    pub const fn new<'a>(label: &'static str, menu_style: MenuStyle<'a, C>) -> CheckboxItem<'a, C> {
        CheckboxItem {
            label,
            highlighted: false,
            position: Point::zero(),
            menu_style,
            checkbox_state: false,
        }
    }
}

impl<C> MenuItemWithData for CheckboxItem<'_, C> where C: PixelColor {}

impl<C: PixelColor> Debug for CheckboxItem<'_, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":Checkbox]", self.label)
    }
}

impl<C: PixelColor> Display for CheckboxItem<'_, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<C: PixelColor> View for CheckboxItem<'_, C> {
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

impl<C: PixelColor> Drawable for CheckboxItem<'_, C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Text::with_baseline(
            self.label,
            self.position,
            self.menu_style.item_character_style,
            Baseline::Top,
        )
        .draw(display)?;

        Text::with_text_style(
            self.display_string(),
            Point::new(display.bounding_box().size().width as i32, 0),
            self.menu_style.item_character_style,
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}

impl<C> MenuItem for CheckboxItem<'_, C>
where
    C: PixelColor,
{
    fn label(&self) -> &'static str {
        self.label
    }
}

impl<C> MenuItemData for CheckboxItem<'_, C>
where
    C: PixelColor,
{
    type MenuItemDataType = bool;

    fn selected(&mut self) -> Self::MenuItemDataType {
        self.checkbox_state = !self.checkbox_state;
        self.checkbox_state
    }

    fn display_string(&self) -> &str {
        match self.checkbox_state {
            true => "[X]",
            false => "[ ]",
        }
    }
}
