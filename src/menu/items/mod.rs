use crate::menu::items::action::ActionItem;
use crate::menu::items::exit_item::ExitItem;
use back_item::BackItem;
use checkbox::CheckboxItem;
use core::fmt::{Display, Formatter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::PixelColor;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;
use embedded_layout::View;
use multi_option::MultiOptionItem;
use section::SectionItem;
use submenu::SubmenuItem;

pub mod action;
pub mod back_item;
pub mod checkbox;
pub mod exit_item;
pub mod multi_option;
pub mod section;
pub mod submenu;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SelectedData<T> {
    Checkbox { id: T, state: bool },
    Submenu { id: T },
    Back { id: T },
    MultiOption { id: T, option_id: usize },
    Section { id: T },
    Action { id: T },
    Exit { id: T },
}

pub trait MenuItem<T>: View + Drawable + DrawableHighlighted + Display + MenuItemData<T> {
    fn label(&self) -> &'static str;

    fn id(&self) -> T;
}

pub trait DrawableHighlighted {
    type Color: PixelColor;
    type Output;

    fn draw_highlighted<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

pub trait MenuItemData<T> {
    fn selected(&mut self) -> SelectedData<T>;

    fn display_string(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItems<'a, C, T>
where
    C: PixelColor,
    T: Clone + Copy + Sized,
{
    Checkbox(CheckboxItem<'a, C, T>),
    Submenu(SubmenuItem<'a, C, T>),
    Selector(MultiOptionItem<'a, C, T>),
    Section(SectionItem<'a, C, T>),
    Back(BackItem<'a, C, T>),
    Action(ActionItem<'a, C, T>),
    Exit(ExitItem<'a, C, T>),
}

impl<C, T> View for MenuItems<'_, C, T>
where
    C: PixelColor,
    T: Copy + Clone + Sized,
{
    fn translate_impl(&mut self, by: Point) {
        match self {
            MenuItems::Checkbox(item) => item.translate_impl(by),
            MenuItems::Submenu(item) => item.translate_impl(by),
            MenuItems::Selector(item) => item.translate_impl(by),
            MenuItems::Section(item) => item.translate_impl(by),
            MenuItems::Back(item) => item.translate_impl(by),
            MenuItems::Action(item) => item.translate_impl(by),
            MenuItems::Exit(item) => item.translate_impl(by),
        }
    }

    fn bounds(&self) -> Rectangle {
        match self {
            MenuItems::Checkbox(item) => item.bounds(),
            MenuItems::Submenu(item) => item.bounds(),
            MenuItems::Selector(item) => item.bounds(),
            MenuItems::Section(item) => item.bounds(),
            MenuItems::Back(item) => item.bounds(),
            MenuItems::Action(item) => item.bounds(),
            MenuItems::Exit(item) => item.bounds(),
        }
    }
}

impl<C, T> Display for MenuItems<'_, C, T>
where
    C: PixelColor,
    T: Copy + Clone + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MenuItems::Checkbox(item) => Display::fmt(&item, f),
            MenuItems::Submenu(item) => Display::fmt(&item, f),
            MenuItems::Selector(item) => Display::fmt(&item, f),
            MenuItems::Section(item) => Display::fmt(&item, f),
            MenuItems::Back(item) => Display::fmt(&item, f),
            MenuItems::Action(item) => Display::fmt(&item, f),
            MenuItems::Exit(item) => Display::fmt(&item, f),
        }
    }
}

impl<C, T> MenuItemData<T> for MenuItems<'_, C, T>
where
    C: PixelColor,
    T: Copy + Clone + Sized,
{
    fn selected(&mut self) -> SelectedData<T> {
        match self {
            MenuItems::Checkbox(item) => item.selected(),
            MenuItems::Submenu(item) => item.selected(),
            MenuItems::Selector(item) => item.selected(),
            MenuItems::Section(item) => item.selected(),
            MenuItems::Back(item) => item.selected(),
            MenuItems::Action(item) => item.selected(),
            MenuItems::Exit(item) => item.selected(),
        }
    }

    fn display_string(&self) -> &str {
        match self {
            MenuItems::Checkbox(item) => item.display_string(),
            MenuItems::Submenu(item) => item.display_string(),
            MenuItems::Selector(item) => item.display_string(),
            MenuItems::Section(item) => item.display_string(),
            MenuItems::Back(item) => item.display_string(),
            MenuItems::Action(item) => item.display_string(),
            MenuItems::Exit(item) => item.display_string(),
        }
    }
}

impl<C, T> MenuItem<T> for MenuItems<'_, C, T>
where
    C: PixelColor,
    T: Copy + Clone + Sized,
{
    fn label(&self) -> &'static str {
        match self {
            MenuItems::Checkbox(item) => item.label(),
            MenuItems::Submenu(item) => item.label(),
            MenuItems::Selector(item) => item.label(),
            MenuItems::Section(item) => item.label(),
            MenuItems::Back(item) => item.label(),
            MenuItems::Action(item) => item.label(),
            MenuItems::Exit(item) => item.label(),
        }
    }

    fn id(&self) -> T {
        match self {
            MenuItems::Checkbox(item) => item.id(),
            MenuItems::Submenu(item) => item.id(),
            MenuItems::Selector(item) => item.id(),
            MenuItems::Section(item) => item.id(),
            MenuItems::Back(item) => item.id(),
            MenuItems::Action(item) => item.id(),
            MenuItems::Exit(item) => item.id(),
        }
    }
}

impl<C, T> Drawable for MenuItems<'_, C, T>
where
    C: PixelColor,
    T: Copy + Clone + Sized,
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
            MenuItems::Back(item) => item.draw(display),
            MenuItems::Action(item) => item.draw(display),
            MenuItems::Exit(item) => item.draw(display),
        }
    }
}

impl<C: PixelColor, T> DrawableHighlighted for MenuItems<'_, C, T>
where
    T: Copy + Clone + Sized,
{
    type Color = C;
    type Output = ();

    fn draw_highlighted<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self {
            MenuItems::Checkbox(item) => item.draw_highlighted(display),
            MenuItems::Submenu(item) => item.draw_highlighted(display),
            MenuItems::Selector(item) => item.draw_highlighted(display),
            MenuItems::Section(item) => item.draw_highlighted(display),
            MenuItems::Back(item) => item.draw_highlighted(display),
            MenuItems::Action(item) => item.draw_highlighted(display),
            MenuItems::Exit(item) => item.draw_highlighted(display),
        }
    }
}
