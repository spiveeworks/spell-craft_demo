use std::rc;

use charm_internal::entities::effects;
use charm_internal::entities::presets;
use charm_internal::entities::spaces;
use charm_internal::events;
use charm_internal::physics;
use charm_internal::units;

use charm_internal::prelude::*;

use game::user_input;

struct PlayerEffect;

impl effects::Effect for PlayerEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0xFF, 0x00, 0x00, 0xFF]
    }
}

// TODO clean up pubs... surely i can private things up a bit better
pub struct Player {
    pub shape: effects::Circle,
    pub body: physics::Body,
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


pub struct GameState {
    pub time: events::EventQueue,
    pub space: Owned<spaces::Space>,
    pub player: Player,
    action: rc::Rc<effects::Cast>,
}

impl GameState {
    pub fn new() -> GameState {
        let time = events::EventQueue::new();

        let space_val = spaces::Space::new();
        let space = Owned::new(space_val);

        let player = Player::new();

        let action = presets::grenade();

        GameState { time, space, player, action }
    }



    pub fn fire(&mut self, target: units::Position) {
        effects::Cast::cast(
            &*self.action,
            &mut self.time,
            self.space.share(),
            self.player.body.clone(),
            target,
        );
    }


    // TODO make DeviceAction enum
    pub fn update_movement(&mut self, dirs: &user_input::DirPad<bool>) {
        let mut x = 0;
        let mut y = 0;

        let speed = self.player.speed;
        if dirs.left  { x -= speed }
        if dirs.right { x += speed }
        if dirs.up    { y -= speed }
        if dirs.down  { y += speed }

        if x != 0 && y != 0 {
            x *= 5;
            x /= 7;
            y *= 5;
            y /= 7;
        }

        let now = self.time.now();
        self.player.body.bounce(units::Vec2 { x, y }, now);
    }
}



