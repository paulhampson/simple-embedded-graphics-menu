use crate::menu::items::{DrawableHighlighted, MenuItem, MenuItemData, SelectedData};
use crate::menu::MenuStyle;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(PartialEq, Clone, Copy)]
pub struct CheckboxItem<'a, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    label: &'static str,
    position: Point,
    menu_style: MenuStyle<'a, C>,
    checkbox_state: bool,
    id: T,
}

impl<C, T> CheckboxItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    pub const fn new<'a>(
        label: &'static str,
        id: T,
        menu_style: MenuStyle<'a, C>,
    ) -> CheckboxItem<'a, C, T> {
        let initial_state = false;
        CheckboxItem {
            label,
            position: Point::zero(),
            menu_style,
            checkbox_state: initial_state,
            id,
        }
    }
}

impl<C, T> MenuItem<T> for CheckboxItem<'_, C, T>
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

impl<C: PixelColor, T> Debug for CheckboxItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":Checkbox]", self.label)
    }
}

impl<C: PixelColor, T> Display for CheckboxItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<C: PixelColor, T> View for CheckboxItem<'_, C, T>
where
    T: Copy + Clone + Sized,
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

impl<C: PixelColor, T> Drawable for CheckboxItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
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

impl<C: PixelColor, T> DrawableHighlighted for CheckboxItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    type Color = C;
    type Output = ();

    fn draw_highlighted<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let highlight_box_style = PrimitiveStyleBuilder::new()
            .fill_color(self.menu_style.highlight_item_color)
            .build();

        Rectangle::new(
            self.position,
            Size::new(
                display.bounding_box().size().width,
                self.menu_style.highlight_text_style.line_height(),
            ),
        )
        .into_styled(highlight_box_style)
        .draw(display)?;

        Text::with_baseline(
            self.label,
            self.position,
            self.menu_style.highlight_text_style,
            Baseline::Top,
        )
        .draw(display)?;

        Text::with_text_style(
            self.display_string(),
            Point::new(display.bounding_box().size().width as i32, 0),
            self.menu_style.highlight_text_style,
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}

impl<C, T> MenuItemData<T> for CheckboxItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    fn selected(&mut self) -> SelectedData<T> {
        self.checkbox_state = !self.checkbox_state;
        SelectedData::Checkbox {
            id: self.id,
            state: self.checkbox_state,
        }
    }

    fn display_string(&self) -> &str {
        match self.checkbox_state {
            true => "[X]",
            false => "[ ]",
        }
    }
}
