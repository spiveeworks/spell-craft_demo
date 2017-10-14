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


