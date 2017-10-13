extern crate piston_window;

use piston_window::*;

fn settings() -> WindowSettings {
    WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
}

fn main() {
    let mut window: PistonWindow =
        settings().exit_on_esc(true)
                  .build()
                  .expect("Failed to build window from settings.");
    while let Some(_e) = window.next() {

    }
}
