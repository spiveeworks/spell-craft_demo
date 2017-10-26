use charm_internal::forms::effects;
use charm_internal::units;

use piston_window as app;

pub trait Draw {
    fn draw(self: &Self, trans: app::math::Matrix2d, graphics: &mut app::G2d);
}

pub fn draw_at<G: Draw>(
    draw: &G,
    position: units::Position,
    center: app::math::Matrix2d,
    graphics: &mut app::G2d
) {
    // methods for operating on our 2d matrices
    use piston_window::Transformed;

    let pos = floatify_position(position);
    let trans = center.trans(pos[0], pos[1]);
    draw.draw(trans, graphics);
}

fn floatify_position(position: units::Position) -> [f64; 2] {
    let units::Vec2 {x, y} = position;
    let x = x as f64 / units::DOT as f64;
    let y = y as f64 / units::DOT as f64;
    [x, y]
}

fn collect_quadruple<T, I>(iter: I) -> Result<[T; 4], Vec<T>>
    where I: Iterator<Item=T>
{
    let vec: Vec<T> = iter.collect();
    if vec.len() == 4 {
        let mut vals = vec.into_iter();
        let el0 = vals.next().unwrap();
        let el1 = vals.next().unwrap();
        let el2 = vals.next().unwrap();
        let el3 = vals.next().unwrap();
        Ok([el0, el1, el2, el3])
    } else {
        Err(vec)
    }
}

fn floatify_color(bytes: [u8; 4]) -> [f32; 4] {
    let floats = bytes
        .iter()
        .cloned()
        .map(|byte| byte as f32 / 256.0);
    collect_quadruple(floats).unwrap()
}

impl Draw for effects::Circle {
    fn draw(self: &Self, trans: app::math::Matrix2d, graphics: &mut app::G2d) {
        let ucolor = effects::Effect::color(&*self.color);
        let fcolor = floatify_color(ucolor);

        let radius = self.radius as f64 / units::DOT as f64;
        // we could transform but this seems clearer
        let rect = [-radius, -radius, 2.0 * radius, 2.0 * radius];

        app::ellipse(fcolor, rect, trans, graphics);
    }
}




