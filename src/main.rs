//! # Example: Hello world
//!
//! A simple hello world example displaying some primitive shapes and some text underneath.

#![no_std]

mod menu;

use crate::menu::menu::MenuItemType::Submenu;
use crate::menu::menu::{print_menu_tree, Menu, MenuItem, MenuItemType};
use embedded_graphics::mono_font::ascii::FONT_6X13_BOLD;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn test_menu(display: &mut SimulatorDisplay<BinaryColor>) {
    let display_size = display.size();
    let heading_style = MonoTextStyle::new(&FONT_6X13_BOLD, BinaryColor::On);
    let item_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let mut menu_root = Menu::new("M1", Point::zero(), display_size, heading_style, item_style);
    menu_root.add_checkbox("M1 Check 1");
    menu_root.add_selector("M1 Selector 1");

    let mut sm = Menu::new(
        "M1-1",
        Point::zero(),
        display_size,
        heading_style,
        item_style,
    );
    sm.add_checkbox("M1-1 Check 1");
    menu_root.add_submenu(sm);

    let mut sm = Menu::new(
        "M1-2",
        Point::zero(),
        display_size,
        heading_style,
        item_style,
    );
    sm.add_checkbox("M1-2 Check 1");
    menu_root.add_submenu(sm);

    print_menu_tree(&menu_root.into());
}

fn main() -> Result<(), core::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    test_menu(&mut display);

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let yoffset = 14;

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display)?;

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)?;

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    // Draw centered text.
    let text = "embedded-graphics";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
