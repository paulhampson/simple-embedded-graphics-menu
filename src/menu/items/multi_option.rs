use crate::menu::items::{DrawableHighlighted, MenuItem, MenuItemData, SelectedData};
use crate::menu::MenuStyle;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::{Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(PartialEq, Clone, Copy)]
pub struct MultiOptionItem<'a, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    label: &'static str,
    highlighted: bool,
    position: Point,
    menu_style: MenuStyle<'a, C>,
    current_option_index: usize,
    options: &'a [&'static str],
    id: T,
}

impl<C, T> MultiOptionItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    pub const fn new<'a>(
        label: &'static str,
        id: T,
        menu_style: MenuStyle<'a, C>,
        options: &'a [&'static str],
    ) -> MultiOptionItem<'a, C, T> {
        let initial_index = 0;
        MultiOptionItem {
            label,
            highlighted: false,
            position: Point::zero(),
            menu_style,
            current_option_index: initial_index,
            options,
            id,
        }
    }
}

impl<C, T> MenuItem<T> for MultiOptionItem<'_, C, T>
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

impl<C: PixelColor, T> Debug for MultiOptionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":MultiOption]", self.label)
    }
}

impl<C: PixelColor, T> Display for MultiOptionItem<'_, C, T>
where
    T: Clone + Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<C: PixelColor, T> View for MultiOptionItem<'_, C, T>
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

impl<C: PixelColor, T> Drawable for MultiOptionItem<'_, C, T>
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

impl<C: PixelColor, T> DrawableHighlighted for MultiOptionItem<'_, C, T>
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

impl<C, T> MenuItemData<T> for MultiOptionItem<'_, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    fn selected(&mut self) -> SelectedData<T> {
        self.current_option_index += 1;
        if self.current_option_index >= self.options.len() {
            self.current_option_index = 0;
        }
        SelectedData::MultiOption {
            id: self.id,
            option_id: self.current_option_index,
        }
    }

    fn display_string(&self) -> &str {
        self.options[self.current_option_index]
    }
}
