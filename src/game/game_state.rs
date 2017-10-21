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
    space: Owned<entities::Space>,
    player: Player,
}

impl GameState {
    fn new() -> GameState {
        let time = events::EventQueue::new();

        let space_val = spaces::Space::new();
        let space = Owned::new(space);

        let player = Player::new();

        GameState { time, space, player }
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

    fn fire(&mut self) {
        let time_now = self.igt.now();
        let _result = entities::Grenade::new(
            &mut self.igt,
            &self.space.share(),
            self.player.body.position(time_now),
            self.cursor_pos,
            1 * units::SEC
        );
    }

    // what module should this be in?
    fn draw(
        self: &mut Self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs,
    ) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context.transform
                            .trans(
                                (ren.width / 2) as f64,
                                (ren.height / 2) as f64
                            );
        let red = [1.0, 0.0, 0.0, 1.0];
        ellipse(
            red,
            rectangle(&self.player.body, self.player.radius, self.igt.now()),
            center,
            graphics
        );
        let space = self.space.share();
        if let Ok(space) = space.try_borrow() {
            for nade in &space.nades {
                let nade = nade.share();
                if let Ok(nade) = nade.try_borrow() {
                    let (rad, col) = match nade.state {
                        entities::GrenadeState::Cooking{..} =>
                            (7.0, [0.0, 0.2, 0.0, 1.0]),
                        entities::GrenadeState::Smoke =>
                            (150.0, [1.0, 0.8, 0.0, 1.0]),
                    };
                    ellipse(
                        col,
                        rectangle(&nade.body, rad, self.igt.now()),
                        center,
                        graphics
                    );
                }
                std::mem::drop(nade);
            }
        }
        std::mem::drop(space);
    }
}


fn rectangle(body: &physics::Body, radius: f64, now: units::Time) -> [f64; 4] {
    let units::Vec2{x, y} = body.position(now);
    let x = x as f64 / units::DOT as f64;
    let y = y as f64 / units::DOT as f64;
    [x - radius, y - radius,
        2.0 * radius, 2.0 * radius]
}
