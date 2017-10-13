extern crate piston_window;

use piston_window::*;

mod game;

struct Player {
    body: game::Body,
    radius: f64,
}

impl Player {
    fn rectangle(&self, now: game::Time) -> [f64; 4] {
        let game::Vec2{x, y} = self.body.position(now);
        let x = x as f64;
        let y = y as f64;
        [x - self.radius, y - self.radius,
         x + self.radius, y + self.radius]
    }
}

struct Game {
    time: game::Time,
    player: Player,
}

impl Game {
    fn new(player_loc: game::Position) -> Game {
        let time = 0;
        let body = game::Body::new(
            player_loc,      // initial location
            game::ZERO_VEC,  // stationary
            time             // any time works since stationary
        );
        let player = Player {
            body,
            radius: 10.0,
        };
        Game { time, player }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.time += (game::SEC as f64 * upd.dt) as game::Time;
    }

    fn on_draw(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs
    ) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context.transform
                            .trans(
                                (ren.width / 2) as f64,
                                (ren.height / 2) as f64
                            );
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(
            red,
            self.player.rectangle(self.time),
            center,
            graphics
        );
    }
}


fn settings() -> WindowSettings {
    WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
}

fn main() {
    let mut game_state = Game::new(game::ZERO_VEC);
    let mut window: PistonWindow =
        settings().exit_on_esc(true)
                  .build()
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
    }
}
