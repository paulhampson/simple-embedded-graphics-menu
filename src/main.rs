//! # Example: Hello world
//!
//! A simple hello world example displaying some primitive shapes and some text underneath.

#![no_std]

mod menu;

use crate::menu::items::MenuItems;
use crate::menu::{Menu, MenuStyle};
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13_BOLD};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use print_no_std::println;

fn test_menu(display: &mut SimulatorDisplay<BinaryColor>) {
    let heading_style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
    let item_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let highlighted_item_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);

    let menu_style = MenuStyle::new(
        heading_style,
        item_style,
        BinaryColor::On,
        BinaryColor::On,
        highlighted_item_style,
    );

    let mut menu_root = Menu::new("M1 Heading", menu_style);
    menu_root.add_checkbox("M1 Check 1");
    let options = &["a", "b", "c"];
    menu_root.add_selector("M1 Selector 1", options);
    menu_root.add_section("Section 1");

    let mut sm = Menu::new("M1-1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", menu_style);
    sm.add_checkbox("M1-1 Check 1");
    menu_root.add_submenu(sm);

    let mut sm = Menu::new("M1-2", menu_style);
    sm.add_checkbox("M1-2 Check 1");
    menu_root.add_submenu(sm);

    menu_root.add_checkbox("M1 Check 2");
    let options = &["c", "d", "e"];
    menu_root.add_selector("M1 Selector 2", options);
    let options = &["f", "g", "h"];
    menu_root.add_selector("M1 Selector 3", options);

    let _ = menu_root.draw(display);
    let menu_tree: trees::Tree<MenuItems<BinaryColor>> = menu_root.into();
    println!("{:?}", menu_tree);
}

fn main() -> Result<(), core::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    test_menu(&mut display);

    // // Create styles used by the drawing operations.
    // let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    // let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    // let border_stroke = PrimitiveStyleBuilder::new()
    //     .stroke_color(BinaryColor::On)
    //     .stroke_width(3)
    //     .stroke_alignment(StrokeAlignment::Inside)
    //     .build();
    // let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    // let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    //
    // let yoffset = 14;
    //
    // // Draw a 3px wide outline around the display.
    // display
    //     .bounding_box()
    //     .into_styled(border_stroke)
    //     .draw(&mut display)?;
    //
    // // Draw a triangle.
    // Triangle::new(
    //     Point::new(16, 16 + yoffset),
    //     Point::new(16 + 16, 16 + yoffset),
    //     Point::new(16 + 8, yoffset),
    // )
    // .into_styled(thin_stroke)
    // .draw(&mut display)?;
    //
    // // Draw a filled square
    // Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
    //     .into_styled(fill)
    //     .draw(&mut display)?;
    //
    // // Draw a circle with a 3px wide stroke.
    // Circle::new(Point::new(88, yoffset), 17)
    //     .into_styled(thick_stroke)
    //     .draw(&mut display)?;
    //
    // // Draw centered text.
    // let text = "embedded-graphics";
    // Text::with_alignment(
    //     text,
    //     display.bounding_box().center() + Point::new(0, 15),
    //     character_style,
    //     Alignment::Center,
    // )
    // .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    Window::new("Menu test", &output_settings).show_static(&display);

    Ok(())
}
