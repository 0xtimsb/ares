use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;

pub fn create_window() -> Result<(), Box<dyn std::error::Error>> {
    let (xcb_connection, screen_num) = x11rb::connect(None)?;
    let screen = &xcb_connection.setup().roots[screen_num];
    let win_id = xcb_connection.generate_id()?;
    xcb_connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        100,
        100,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;
    xcb_connection.map_window(win_id)?;
    xcb_connection.flush()?;
    loop {
        println!("Event: {:?}", xcb_connection.wait_for_event()?);
    }
}
