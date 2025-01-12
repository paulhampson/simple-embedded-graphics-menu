use crate::menu::items::checkbox::CheckboxItem;
use crate::menu::items::multi_option::MultiOptionItem;
use crate::menu::items::section::SectionItem;
use crate::menu::items::submenu::SubmenuItem;
use core::fmt::{Display, Formatter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::PixelColor;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;
use embedded_layout::View;

pub mod checkbox;
pub mod multi_option;
pub mod section;
pub mod submenu;

pub trait MenuItem: View + Drawable + Display {
    fn label(&self) -> &'static str;
}

pub trait MenuItemWithData: MenuItem + MenuItemData {}

pub trait MenuItemData {
    type MenuItemDataType;

    fn selected(&mut self) -> Self::MenuItemDataType;

    fn display_string(&self) -> &str;
}

#[derive(Debug, Clone, Copy)]
pub enum MenuItems<'a, C>
where
    C: PixelColor,
{
    Checkbox(CheckboxItem<'a, C>),
    Submenu(SubmenuItem<'a, C>),
    Selector(MultiOptionItem<'a, C>),
    Section(SectionItem<'a, C>),
}

impl<C> View for MenuItems<'_, C>
where
    C: PixelColor,
{
    fn translate_impl(&mut self, by: Point) {
        match self {
            MenuItems::Checkbox(item) => item.translate_impl(by),
            MenuItems::Submenu(item) => item.translate_impl(by),
            MenuItems::Selector(item) => item.translate_impl(by),
            MenuItems::Section(item) => item.translate_impl(by),
        }
    }

    fn bounds(&self) -> Rectangle {
        match self {
            MenuItems::Checkbox(item) => item.bounds(),
            MenuItems::Submenu(item) => item.bounds(),
            MenuItems::Selector(item) => item.bounds(),
            MenuItems::Section(item) => item.bounds(),
        }
    }
}

impl<C> Display for MenuItems<'_, C>
where
    C: PixelColor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MenuItems::Checkbox(item) => Display::fmt(&item, f),
            MenuItems::Submenu(item) => Display::fmt(&item, f),
            MenuItems::Selector(item) => Display::fmt(&item, f),
            MenuItems::Section(item) => Display::fmt(&item, f),
        }
    }
}

impl<C> MenuItem for MenuItems<'_, C>
where
    C: PixelColor,
{
    fn label(&self) -> &'static str {
        match self {
            MenuItems::Checkbox(item) => item.label(),
            MenuItems::Submenu(item) => item.label(),
            MenuItems::Selector(item) => item.label(),
            MenuItems::Section(item) => item.label(),
        }
    }
}

impl<C> Drawable for MenuItems<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self {
            MenuItems::Checkbox(item) => item.draw(display),
            MenuItems::Submenu(item) => item.draw(display),
            MenuItems::Selector(item) => item.draw(display),
            MenuItems::Section(item) => item.draw(display),
        }
    }
}
