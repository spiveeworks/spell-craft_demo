use std::ops;


pub type Time = i64;
pub type Scalar = i64;

pub const SEC: Time = 1 << 16; // 65536
// a DOT is merely the distance traveled in a SEC with a velocity of 1
// in this way it is a useful stepping stone
// it represents something close to the minimum reasonable distance
// when working with velocities
pub const DOT: Scalar = SEC;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

// could use the Zero trait
pub const ZERO_VEC: Vec2 = Vec2 {
    x: 0,
    y: 0,
};

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(mut self: Vec2, v2: Vec2) -> Vec2 {
        self += v2;
        self
    }
}

impl ops::MulAssign<Scalar> for Vec2 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<Scalar> for Vec2 {
    type Output = Vec2;
    fn mul(mut self: Vec2, t: Scalar) -> Vec2 {
        self *= t;
        self
    }
}

impl ops::Mul<Vec2> for Scalar {
    type Output = Vec2;
    fn mul(self: Scalar, v: Vec2) -> Vec2 {
        v * self
    }
}


// these could be newtypes, which would catch value-handling errors for us
// but that would mean a lot of redundant trait implementations, so it can
// wait.
pub type Position = Vec2;
pub type Displacement = Vec2;
pub type Velocity = Vec2;


#[derive(Clone, Debug)]
pub struct Body {
    last_position: Position,
    current_velocity: Velocity,
    last_time: Time,
}

impl Body {
    pub fn new(
        position: Position,
        velocity: Velocity,
        time: Time
    ) -> Body {
        Body {
            last_position: position,
            current_velocity: velocity,
            last_time: time,
        }
    }

    pub fn position(&self, now: Time) -> Position {
        let dtime = now - self.last_time;
        let displacement = self.current_velocity * dtime;
        self.last_position + displacement
    }

    pub fn velocity(&self) -> Velocity {
        self.current_velocity
    }

    pub fn bounce(&self, velocity: Velocity, now: Time) -> Body {
        Body {
            last_position: self.position(now),
            current_velocity: velocity,
            last_time: now,
        }
    }

    pub fn update(&mut self, velocity: Velocity, now: Time) {
        *self = self.bounce(velocity, now);
    }

    pub fn freeze(&mut self, now: Time) {
        self.update(Vec2 { x: 0, y: 0 }, now);
    }
}



