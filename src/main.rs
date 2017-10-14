extern crate piston_window;

use piston_window::*;

use std::time;
use std::ops;

extern crate charm_internal;

use charm_internal as game;

const SPEED: game::Scalar = 100;
const MAX_SKIP: game::Time = game::SEC / 16;

struct Player {
    body: game::Body,
    radius: f64,
}

impl Player {
    fn rectangle(&self, now: game::Time) -> [f64; 4] {
        let game::Vec2{x, y} = self.body.position(now);
        let x = x as f64 / game::DOT as f64;
        let y = y as f64 / game::DOT as f64;
        [x - self.radius, y - self.radius,
            2.0 * self.radius, 2.0 * self.radius]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
struct DirPad<T> {
    up: T,
    down: T,
    left: T,
    right: T,
}

impl<T> ops::Index<Dir> for DirPad<T> {
    type Output = T;
    fn index(&self, index: Dir) -> &T {
        match index {
            Dir::Up    => &self.up,
            Dir::Down  => &self.down,
            Dir::Left  => &self.left,
            Dir::Right => &self.right,
        }
    }
}

impl<T> ops::IndexMut<Dir> for DirPad<T> {
    fn index_mut(&mut self, index: Dir) -> &mut T {
        match index {
            Dir::Up    => &mut self.up,
            Dir::Down  => &mut self.down,
            Dir::Left  => &mut self.left,
            Dir::Right => &mut self.right,
        }
    }
}

impl<T> DirPad<T>
    where T: PartialEq
{
    fn dir(&self, item: T) -> Option<Dir> {
        if      item == self.up    { Some( Dir::Up    ) }
        else if item == self.down  { Some( Dir::Down  ) }
        else if item == self.left  { Some( Dir::Left  ) }
        else if item == self.right { Some( Dir::Right ) }
        else { None }
    }
}

fn duration_in_game(duration: time::Duration) -> game::Time {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let time_s = seconds as game::Time * game::SEC;
    // a billion times the actual time represented by the nanos
    let time_n_bi =   nanos as game::Time * game::SEC;
    time_s + time_n_bi / 1_000_000_000
}

struct Clock {
    start_instant: Option<time::Instant>,
    last_time: game::Time,
}

impl Clock {
    fn new() -> Clock {
        Clock {
            start_instant: None,
            last_time: 0,
        }
    }

    fn elapsed_as_of(&self, now: time::Instant) -> time::Duration {
        if let Some(start) = self.start_instant {
            now.duration_since(start)
        } else {
            // time only passes if the clock has started
            time::Duration::new(0,0)
        }
    }

    fn time(&self, now: time::Instant) -> game::Time {
        let elapsed = self.elapsed_as_of(now);
        self.last_time + duration_in_game(elapsed)
    }

    fn stop(&mut self, now: time::Instant) {
        self.last_time = self.time(now);
        self.start_instant = None;
    }

    fn start(&mut self, now: time::Instant) {
        self.stop(now);
        self.start_instant = Some(now);
    }
}

struct Game {
    clock: Clock,
    current_time: game::Time,
    last_render: game::Time,
    player: Player,
    controls: DirPad<Button>,
    dirs: DirPad<bool>,
}

impl Game {
    fn new(player_loc: game::Position) -> Game {
        let mut clock = Clock::new();
        clock.start(time::Instant::now());

        let initial_time = clock.last_time;

        let body = game::Body::new(
            player_loc,      // initial location
            game::ZERO_VEC,  // stationary
            initial_time,    // any time works since stationary
        );

        let player = Player {
            body,
            radius: 10.0,
        };

        let controls = DirPad {
            up:    Button::Keyboard(Key::W),
            down:  Button::Keyboard(Key::S),
            left:  Button::Keyboard(Key::A),
            right: Button::Keyboard(Key::D),
        };

        let dirs = Default::default();

        Game {
            clock,
            current_time: initial_time,
            last_render: initial_time,
            player,
            controls,
            dirs,
        }
    }

    fn on_update(&mut self, _upd: UpdateArgs) {
        let now = time::Instant::now();
        self.current_time = self.clock.time(now);

        // maximum in-game time before rendering again
        let max_time = self.last_render + MAX_SKIP;
        if self.current_time > max_time {
            self.current_time = max_time;
            self.clock = Clock {
                start_instant: Some(now),
                last_time: max_time,
            }
        }
    }

    fn update_movement(&mut self, dir: Dir, state: bool) {
        if self.dirs[dir] == state {
            // short circuit to avoid unnecessary rounding
            return;
        }
        self.dirs[dir] = state;

        let mut x = 0;
        let mut y = 0;
        if self.dirs.left  { x -= SPEED }
        if self.dirs.right { x += SPEED }
        if self.dirs.up    { y -= SPEED }
        if self.dirs.down  { y += SPEED }

        if x != 0 && y != 0 {
            x *= 5;
            x /= 7;
            y *= 5;
            y /= 7;
        }

        self.player.body.update(game::Vec2 { x, y }, self.current_time);
    }

    fn on_input(&mut self, bin: ButtonArgs) {
        let ButtonArgs { button, state, .. } = bin;
        let state = state == ButtonState::Press;  // true if pressed

        if let Some(dir) = self.controls.dir(button) {
            self.update_movement(dir, state);
        }
    }

    fn on_draw(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs
    ) {
        self.last_render = self.current_time;
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context.transform
                            .trans(
                                (ren.width / 2) as f64,
                                (ren.height / 2) as f64
                            );
        let red = [1.0, 0.0, 0.0, 1.0];
        ellipse(
            red,
            self.player.rectangle(self.current_time),
            center,
            graphics
        );
    }
}


fn settings() -> WindowSettings {
    WindowSettings::new(
        "charm-game",
        [600, 600]
    )
}

fn main() {
    let mut game_state = Game::new(game::ZERO_VEC);
    game_state.clock.start(time::Instant::now());

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
        if let Some(bin) = e.button_args() {
            game_state.on_input(bin);
        }
    }
}
