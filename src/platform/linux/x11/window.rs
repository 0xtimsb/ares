use std::error::Error;

use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xproto::ConnectionExt as XProtoConnectionExt;
use x11rb::protocol::{xproto::*, Event};
use x11rb::wrapper::ConnectionExt;

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

fn button_draw<C: Connection>(
    conn: &C,
    screen: &Screen,
    window: Window,
    x1: i16,
    y1: i16,
    label: &str,
) -> Result<(), ReplyOrIdError> {
    let inset = 2;
    let gc = gc_font_get(conn, screen, window, "7x13")?;
    let width = 7 * label.len() + 2 * (inset + 1);
    let height = 13 + 2 * (inset + 1);
    let (width, height) = (width as i16, height as i16);
    let inset = inset as i16;

    let points = [
        Point { x: x1, y: y1 },
        Point {
            x: x1 + width,
            y: y1,
        },
        Point {
            x: x1 + width,
            y: y1 - height,
        },
        Point {
            x: x1,
            y: y1 - height,
        },
        Point { x: x1, y: y1 },
    ];
    conn.poly_line(CoordMode::ORIGIN, window, gc, &points)?;
    conn.image_text8(window, gc, x1 + inset + 1, y1 - inset - 1, label.as_bytes())?;
    conn.free_gc(gc)?;
    Ok(())
}

fn cursor_set<C: Connection>(
    conn: &C,
    screen: &Screen,
    window: Window,
    cursor_id: u16,
) -> Result<(), ReplyOrIdError> {
    let font = conn.generate_id()?;
    conn.open_font(font, b"cursor")?;

    let cursor = conn.generate_id()?;
    conn.create_glyph_cursor(
        cursor,
        font,
        font,
        cursor_id,
        cursor_id + 1,
        0,
        0,
        0,
        0,
        0,
        0,
    )?;

    let gc = conn.generate_id()?;
    let values = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .background(screen.black_pixel)
        .font(font);
    conn.create_gc(gc, window, &values)?;

    let values = ChangeWindowAttributesAux::default().cursor(cursor);
    conn.change_window_attributes(window, &values)?;

    conn.free_cursor(cursor)?;
    conn.close_font(font)?;
    Ok(())
}

pub fn create_window() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    const WIDTH: i16 = 300;
    const HEIGHT: i16 = 100;

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
        WIDTH as u16,
        HEIGHT as u16,
        10,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &values,
    )?;

    let title = "Hello World !";
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )?;

    let title_icon = "Hello World ! (iconified)";
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_ICON_NAME,
        AtomEnum::STRING,
        title_icon.as_bytes(),
    )?;

    conn.map_window(window)?;

    cursor_set(&conn, screen, window, 68)?;

    conn.flush()?;

    let mut is_hand = false;

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(event) => {
                let text = "Press ESC key to exit...";
                text_draw(&conn, screen, window, 10, HEIGHT as i16 - 10, text)?;
                conn.flush()?;
                let text = "click here to change cursor";
                button_draw(
                    &conn,
                    screen,
                    window,
                    (WIDTH - 7 * text.len() as i16) / 2,
                    (HEIGHT - 16) / 2,
                    text,
                )?;

                let text = "Press ESC key to exit...";
                text_draw(&conn, screen, window, 10, HEIGHT - 10, text)?;
                conn.flush()?;
            }
            Event::ButtonPress(event) => {
                let length = "click here to change cursor".len() as i16;

                if (event.event_x >= (WIDTH - 7 * length) / 2)
                    && (event.event_x <= ((WIDTH - 7 * length) / 2 + 7 * length + 6))
                    && (event.event_y >= (HEIGHT - 16) / 2 - 19)
                    && (event.event_y <= ((HEIGHT - 16) / 2))
                {
                    is_hand = !is_hand;
                }

                if is_hand {
                    cursor_set(&conn, screen, window, 58)?;
                } else {
                    cursor_set(&conn, screen, window, 68)?;
                }
                conn.flush()?;
            }
            Event::KeyRelease(event) => {
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
