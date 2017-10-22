use std::ops;

use charm_internal::units;

use piston_window as app;


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




struct Input {
    move_controls: DirPad<app::Button>,
    fire_button: app::Button,
    dirs: DirPad<bool>,
    cursor_pos: units::Position,
}

impl Input {
    fn new() -> Input {
        let move_controls = DirPad {
            up:    app::Button::Keyboard(app::Key::W),
            down:  app::Button::Keyboard(app::Key::S),
            left:  app::Button::Keyboard(app::Key::A),
            right: app::Button::Keyboard(app::Key::D),
        };
        let fire_button = app::Button::Mouse(app::MouseButton::Left);

        let dirs = Default::default();
        let cursor_pos = units::ZERO_VEC;

        Input {
            move_controls,
            fire_button,
            dirs,
            cursor_pos,
        }
    }

    fn on_input(&mut self, game_state: &mut GameState, bin: ButtonArgs) {
        let ButtonArgs { button, state, .. } = bin;
        let state = state == app::ButtonState::Press;  // true if pressed

        if let Some(dir) = self.move_controls.dir(button) {
            // short circuit to avoid unnecessary rounding
            if self.dirs[dir] != state {
                self.dirs[dir] = state;
                game_state.update_movement(&self.dirs);
            }
        }

        if state && button == self.fire_button {
            game_state.fire(self.cursor_pos);
        }
    }

    fn on_mouse_move(&mut self, mouse: [f64; 2]) {
        let x = (mouse[0] - 300.0) * units::DOT as f64;
        let y = (mouse[1] - 300.0) * units::DOT as f64;
        self.cursor_pos = units::Vec2 {
            x: x as units::Scalar,
            y: y as units::Scalar,
        };
    }
}

