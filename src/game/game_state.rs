use std::rc;

use charm_internal::entities::effects;
use charm_internal::entities::presets;
use charm_internal::entities::spaces;
use charm_internal::events;
use charm_internal::physics;
use charm_internal::units;

use charm_internal::prelude::*;


struct PlayerEffect;

impl effects::Effect for PlayerEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0xFF, 0x00, 0x00, 0xFF]
    }
}

struct Player {
    shape: effects::Circle,
    body: physics::Body,
    speed: units::Scalar,
}

impl Player {
    fn new() -> Player {
        let color = rc::Rc::new(PlayerEffect);
        let radius = 10 * units::DOT;
        let shape = effects::Circle { color, radius };

        let body = physics::Body::new_frozen(units::ZERO_VEC);

        let speed = 100;

        Player { shape, body, speed }
    }
}


struct GameState {
    time: events::EventQueue,
    space: Owned<spaces::Space>,
    player: Player,
    action: rc::Rc<effects::Cast>,
}

impl GameState {
    fn new() -> GameState {
        let time = events::EventQueue::new();

        let space_val = spaces::Space::new();
        let space = Owned::new(space);

        let player = Player::new();

        let action = presets::grenade();

        GameState { time, space, player, action }
    }

    // what module should this be in?
    fn update_movement(&mut self, dir: Dir, state: bool) {


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

        self.player.body.bounce(units::Vec2 { x, y }, self.igt.now());
    }

    fn fire(&mut self, target: units::Position) {
        let time_now = self.igt.now();
        effects::Cast::cast(
            &*self.action,
            &mut self.time,
            self.space.share(),
            self.player.body,
            target,
        );
    }
}



