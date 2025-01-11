use core::cmp::PartialEq;
use core::fmt::Formatter;
use core::{error, fmt};
use embedded_graphics::geometry::AnchorY;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use embedded_layout::View;
use print_no_std::println;
use trees::{Iter, Tree};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MenuError {
    InternalTreeError,
}

impl fmt::Display for MenuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            MenuError::InternalTreeError => {
                write!(f, "menu internal tree error")
            }
        }
    }
}

impl error::Error for MenuError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuItemType {
    Heading,
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

pub struct Menu<'a, C>
where
    C: PixelColor,
{
    menu_tree: Tree<MenuItem<'a, C>>,
    bounds: Rectangle,
    heading_style: MonoTextStyle<'a, C>,
    item_style: MonoTextStyle<'a, C>,
}

impl<'a, C> Drawable for Menu<'a, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let display_area = display.bounding_box();
        let header = self.menu_tree.data();
        let header_height = header.size().height;
        header.draw(display)?;
        let mut remaining_item_area = display_area
            .resized_height(display_area.size().height - header_height, AnchorY::Bottom);

        for menu_item in self.menu_tree.iter() {
            let item_height = menu_item.data().size().height;
            if item_height > remaining_item_area.size().height {
                break;
            }

            let mut item_display = display.cropped(&remaining_item_area);
            menu_item.data().draw(&mut item_display)?;

            remaining_item_area = remaining_item_area.resized_height(
                remaining_item_area.size().height - item_height,
                AnchorY::Bottom,
            );
        }

        Ok(())
    }
}

impl<'a, C> Menu<'a, C>
where
    C: PixelColor,
{
    pub fn new(
        label: &'static str,
        position: Point,
        size: Size,
        heading_style: MonoTextStyle<'a, C>,
        item_style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self {
            menu_tree: Tree::new(MenuItem::new(label, MenuItemType::Submenu, heading_style)),
            bounds: Rectangle::new(position, size),
            heading_style,
            item_style,
        }
    }

    /// Add menu item to the menu structure that will be drawn
    pub fn add_item(&mut self, item: MenuItem<'a, C>) {
        self.menu_tree.push_back(Tree::new(item));
    }

    /// Add checkbox as next item in the menu
    pub fn add_checkbox(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Checkbox, self.item_style);
        self.add_item(item);
    }

    /// Add selector as next item in the menu
    pub fn add_selector(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Selector, self.item_style);
        self.add_item(item);
    }

    /// Add header as next item in the menu
    pub fn add_header(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Heading, self.heading_style);
        self.add_item(item);
    }

    /// Add a sub-menu to the menu structure that will be drawn
    pub fn add_submenu(&mut self, submenu: Menu<'a, C>) {
        self.menu_tree.push_back(submenu.into());
    }
}

impl<'a, C> From<Menu<'a, C>> for Tree<MenuItem<'a, C>>
where
    C: PixelColor,
{
    fn from(menu: Menu<'a, C>) -> Tree<MenuItem<'a, C>> {
        menu.menu_tree
    }
}

pub fn print_menu_tree<C>(menu_tree: &Tree<MenuItem<C>>)
where
    C: PixelColor,
{
    println!("{}", menu_tree.data());

    print_tree_iter(menu_tree.iter())
}

pub fn print_tree_iter<C>(tree_iter: Iter<MenuItem<C>>)
where
    C: PixelColor,
{
    for entry in tree_iter {
        println!("{}", entry.data());
        if entry.data().item_type == MenuItemType::Submenu {
            print_tree_iter(entry.iter()); // the children
        }
    }
}
