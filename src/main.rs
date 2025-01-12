//! # Example: Hello world
//!
//! A simple hello world example displaying some primitive shapes and some text underneath.

#![no_std]

mod menu;

use crate::menu::{Menu, MenuStyle};
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13_BOLD};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn build_menu<'a>() -> Menu<'a, BinaryColor> {
    let heading_style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
    let item_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let highlighted_item_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);

    let menu_style = MenuStyle::new(
        BinaryColor::Off,
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

    let mut sm = Menu::new("M1-1", menu_style);
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

    menu_root
}

fn main() -> Result<(), core::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let mut menu = build_menu();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    let mut window = Window::new("Menu test", &output_settings);

    'gui_update_loop: loop {
        menu.draw(&mut display)?;
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'gui_update_loop,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => menu.navigate_up(),
                        Keycode::Down => menu.navigate_down(),
                        Keycode::Return => menu.select_item(),
                        _ => (),
                    };
                }
                _ => {}
            }
        }
    }

    Ok(())
}
