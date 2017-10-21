extern crate piston_window;

use piston_window::*;

use std::ops;

extern crate charm_internal;

use charm_internal::{units, physics, events, entities};
use charm_internal::prelude::*;





fn settings() -> WindowSettings {
    WindowSettings::new(
        "charm-game",
        [600, 600]
    ).exit_on_esc(true)
}

fn main() {
    let mut game_state = Game::new(units::ZERO_VEC);
    game_state.clock.start(time::Instant::now());

    let mut window: PistonWindow =
        settings().build()
                  .unwrap_or_else(|e| {
                      panic!("Failed to build PistonWindow: {}", e)
                  });
    while let Some(e) = window.next() {
        if let Some(ren) = e.render_args() {
            window.draw_2d(&e, |c, g| game_state.on_draw(c, g, ren));
        }
        if let Some(upd) = e.update_args() {
            game_state.on_update(upd);
        }
        if let Some(bin) = e.button_args() {
            game_state.on_input(bin);
        }
        if let Some(mouse) = e.mouse_cursor_args() {
            game_state.on_mouse_move(mouse);
        }
    }
}
