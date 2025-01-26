#![no_std]

use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13_BOLD};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use log::info;
use simple_embedded_graphics_menu::{Menu, MenuStyle};

fn build_menu<'a>() -> Menu<'a, BinaryColor, i32> {
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
        BinaryColor::Off,
    );

    let mut counter = 0..100;
    let mut menu_root = Menu::new("M1 Heading", counter.next().unwrap_or(0i32), menu_style);
    menu_root.add_section("Section 0", counter.next().unwrap_or(0i32));
    menu_root.add_checkbox("M1 Check 1", counter.next().unwrap_or(0i32));
    let options = &["a0", "b1", "c2"];
    menu_root.add_selector(
        "M1 Selector 1",
        counter.next().unwrap_or(0i32),
        options,
        None,
    );
    menu_root.add_section("Section 1", counter.next().unwrap_or(0i32));

    let mut sm = Menu::new("M1-1", counter.next().unwrap_or(0i32), menu_style);
    sm.add_checkbox("M1-1 Check 1", counter.next().unwrap_or(0i32));
    sm.add_back("Back", counter.next().unwrap_or(0i32));
    menu_root.add_submenu(sm);

    let mut sm = Menu::new("M1-2", counter.next().unwrap_or(0i32), menu_style);
    sm.add_checkbox("M1-2 Check 1", counter.next().unwrap_or(0i32));
    let options = &["m1-2c", "m1-2d", "m1-2e"];
    sm.add_selector(
        "M1-2 Selector 1",
        counter.next().unwrap_or(0i32),
        options,
        Some(34), // too big - will cap at last selector option (m1-2e)
    );
    sm.add_back("Back", counter.next().unwrap_or(0i32));
    menu_root.add_submenu(sm);

    menu_root.add_section("Section 2", counter.next().unwrap_or(0i32));
    menu_root.add_checkbox("M1 Check 2", counter.next().unwrap_or(0i32));
    let options = &["c0", "d1", "e2"];
    menu_root.add_selector(
        "M1 Selector 2",
        counter.next().unwrap_or(0i32),
        options,
        Some(1),
    );
    let options = &["f0", "g1", "h2"];
    menu_root.add_selector(
        "M1 Selector 3",
        counter.next().unwrap_or(0i32),
        options,
        Some(2),
    );
    menu_root.add_action("Action 1", counter.next().unwrap_or(0i32));
    menu_root.add_exit("Exit", counter.next().unwrap_or(0i32));

    menu_root
}

fn main() -> Result<(), core::convert::Infallible> {
    env_logger::init();
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
                        Keycode::Return => {
                            if let Some(selected_data) = menu.select_item() {
                                info!("{:?}", selected_data);
                            }
                        }
                        _ => (),
                    };
                }
                _ => {}
            }
        }
    }

    Ok(())
}
