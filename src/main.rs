extern crate piston_window;

use piston_window::*;

extern crate charm_internal;

mod game;


fn settings() -> WindowSettings {
    WindowSettings::new(
        "charm-game",
        [600, 600]
    ).exit_on_esc(true)
}

fn main() {
    let mut game_data = game::Game::new();


    let mut window: PistonWindow =
        settings().build()
                  .unwrap_or_else(|e| {
                      panic!("Failed to build PistonWindow: {}", e)
                  });
    while let Some(e) = window.next() {
        if let Some(ren) = e.render_args() {
            window.draw_2d(&e, |c, g| game_data.on_draw(c, g, ren));
        }
        if let Some(upd) = e.update_args() {
            game_data.on_update(upd);
        }
        if let Some(bin) = e.button_args() {
            game_data.on_input(bin);
        }
        if let Some(mouse) = e.mouse_cursor_args() {
            game_data.on_mouse_move(mouse);
        }
    }
}
