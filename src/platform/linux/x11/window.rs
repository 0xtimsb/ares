use std::error::Error;

use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::{xproto::*, Event};

fn print_modifiers(mask: x11rb::protocol::xproto::KeyButMask) {
    println!("Modifier mask: {:#?}", mask);
}

fn text_draw<C: Connection>(
    conn: &C,
    screen: &Screen,
    window: Window,
    x1: i16,
    y1: i16,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    let gc = gc_font_get(conn, screen, window, "7x13")?;

    conn.image_text8(window, gc, x1, y1, label.as_bytes())?;
    conn.free_gc(gc)?;

    Ok(())
}

fn gc_font_get<C: Connection>(
    conn: &C,
    screen: &Screen,
    window: Window,
    font_name: &str,
) -> Result<Gcontext, ReplyOrIdError> {
    let font = conn.generate_id()?;

    conn.open_font(font, font_name.as_bytes())?;

    let gc = conn.generate_id()?;
    let values = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .background(screen.white_pixel)
        .font(font);
    conn.create_gc(gc, window, &values)?;

    conn.close_font(font)?;

    Ok(gc)
}

pub fn create_window() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    const WIDTH: u16 = 300;
    const HEIGHT: u16 = 100;

    let font_id = conn.generate_id()?;

    let font_name: &[u8] = b"6x13";
    conn.open_font(font_id, font_name)?;

    let window = conn.generate_id()?;
    let values = CreateWindowAux::default()
        .background_pixel(screen.white_pixel)
        .event_mask(
            EventMask::EXPOSURE
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::ENTER_WINDOW
                | EventMask::LEAVE_WINDOW
                | EventMask::KEY_PRESS
                | EventMask::KEY_RELEASE,
        );
    conn.create_window(
        screen.root_depth,
        window,
        screen.root,
        0,
        0,
        WIDTH,
        HEIGHT,
        10,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &values,
    )?;

    conn.map_window(window)?;
    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(event) => {
                let text = "Press ESC key to exit...";
                text_draw(&conn, screen, window, 10, HEIGHT as i16 - 10, text)?;
                conn.flush()?;
            }
            Event::ButtonPress(event) => {
                print_modifiers(event.state);
                match event.detail {
                    4 => println!(
                        "Wheel Button up in window {}, at coordinates ({},{})",
                        event.event, event.event_x, event.event_y
                    ),
                    5 => println!(
                        "Wheel Button down in window {}, at coordinates ({},{})",
                        event.event, event.event_x, event.event_y
                    ),
                    _ => println!(
                        "Button {} pressed in window {}, at coordinates ({},{})",
                        event.detail, event.event, event.event_x, event.event_y
                    ),
                }
            }
            Event::ButtonRelease(event) => {
                print_modifiers(event.state);
                println!(
                    "Button {} released in window {}, at coordinates ({},{})",
                    event.detail, event.event, event.event_x, event.event_y
                );
            }
            Event::MotionNotify(event) => {
                println!(
                    "Mouse moved in window {} at coordinates ({},{})",
                    event.event, event.event_x, event.event_y
                );
            }
            Event::EnterNotify(event) => {
                println!(
                    "Mouse entered window {} at coordinates ({},{})",
                    event.event, event.event_x, event.event_y
                );
            }
            Event::LeaveNotify(event) => {
                println!(
                    "Mouse left window {} at coordinates ({},{})",
                    event.event, event.event_x, event.event_y
                );
            }
            Event::KeyPress(event) => {
                print_modifiers(event.state);
                println!("Key pressed in window {}", event.event);
            }
            Event::KeyRelease(event) => {
                print_modifiers(event.state);
                println!("Key released in window {}", event.event);

                if event.detail == 9 {
                    // ESC
                    return Ok(());
                }
            }
            _ => {
                // Unknown event type, ignore it
                println!("Unknown event: {:?}", event);
            }
        }
    }
}
