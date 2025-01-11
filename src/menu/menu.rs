use crate::menu::menu_item::{MenuItem, MenuItemType};
use embedded_graphics::geometry::AnchorY;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use embedded_layout::View;
use trees::Tree;

pub struct Menu<'a, C>
where
    C: PixelColor,
{
    menu_tree: Tree<MenuItem<'a, C>>,
    menu_style: MenuStyle<'a, C>,
}

impl<'a, C> Menu<'a, C>
where
    C: PixelColor,
{
    pub fn new(label: &'static str, menu_style: MenuStyle<'a, C>) -> Self {
        Self {
            menu_tree: Tree::new(MenuItem::new(label, MenuItemType::Submenu, menu_style)),
            menu_style,
        }
    }

    /// Add menu item to the menu structure that will be drawn
    pub fn add_item(&mut self, item: MenuItem<'a, C>) {
        self.menu_tree.push_back(Tree::new(item));
    }

    /// Add checkbox as next item in the menu
    pub fn add_checkbox(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Checkbox, self.menu_style);
        self.add_item(item);
    }

    /// Add selector as next item in the menu
    pub fn add_selector(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Selector, self.menu_style);
        self.add_item(item);
    }

    /// Add section (non-selectable item) as next item in the menu
    pub fn add_section(&mut self, label: &'static str) {
        let item = MenuItem::new(label, MenuItemType::Section, self.menu_style);
        self.add_item(item);
    }

    /// Add a sub-menu to the menu structure that will be drawn
    pub fn add_submenu(&mut self, submenu: Menu<'a, C>) {
        self.menu_tree.push_back(submenu.into());
    }
}

impl<C> Drawable for Menu<'_, C>
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
        let header_height = self.menu_style.heading_character_style.line_height();
        Text::with_baseline(
            header.label(),
            Point::zero(),
            self.menu_style.heading_character_style,
            Baseline::Top,
        )
        .draw(display)?;

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

impl<'a, C> From<Menu<'a, C>> for Tree<MenuItem<'a, C>>
where
    C: PixelColor,
{
    fn from(menu: Menu<'a, C>) -> Tree<MenuItem<'a, C>> {
        menu.menu_tree
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MenuStyle<'a, C> {
    pub(crate) heading_character_style: MonoTextStyle<'a, C>,
    pub(crate) item_character_style: MonoTextStyle<'a, C>,
    pub(crate) indicator_fill_color: C,
    pub(crate) highlight_item_color: C,
    pub(crate) highlight_text_style: MonoTextStyle<'a, C>,
}

impl<'a, C> MenuStyle<'a, C>
where
    C: PixelColor,
{
    pub fn new(
        heading_character_style: MonoTextStyle<'a, C>,
        item_character_style: MonoTextStyle<'a, C>,
        indicator_fill_color: C,
        highlight_item_color: C,
        highlight_text_style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self {
            heading_character_style,
            item_character_style,
            indicator_fill_color,
            highlight_item_color,
            highlight_text_style,
        }
    }
}
