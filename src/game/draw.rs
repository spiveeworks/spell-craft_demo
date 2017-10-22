use charm_internal::entities::effects;
use charm_internal::units;

use piston_window as app;

pub trait Draw {
    fn draw(self: &Self, trans: app::math::Matrix2d, graphics: &mut app::G2d);
}

fn floatify_position(position: units::Position) -> [f64; 2] {
    let units::Vec2{x, y} = body.position(now);
    let x = x as f64 / units::DOT as f64;
    let y = y as f64 / units::DOT as f64;
    [x, y]
}

fn draw_at<G: Draw>(
    draw: &G,
    position: units::Position,
    center: app::math::Matrix2d,
    graphics: &mut app::G2d
) {
    let pos = floatify_position(position);
    let trans = center.trans(pos[0], pos[1]);
    draw.draw(trans, graphics);
}

fn collect_quadruple<T, I>(iter: I) -> Result<[T; 4], Vec<T>>
    where I: Iterator<Item=T>
{
    let vec = iter.collect();
    if (vec.len() == 4) {
        Ok([vec[0], vec[1], vec[2], vec[3]])
    } else {
        Err(vec)
    }
}

fn floatify_color(bytes: [u8; 4]) -> [f64; 4] {
    let floats = bytes
        .iter()
        .cloned()
        .map(|byte| byte as f64 / 256.0);
    collect_quadruple(floats).unwrap()
}

impl Draw for effects::Circle {
    fn draw(self: &Self, trans: app::Matrix2d, graphics: &mut app::G2d) {
        let ucolor = effects::Effect::color(&*self.color);
        let fcolor = floatify_color(color);

        let radius = self.radius as f64 / units::DOT as f64;
        // we could transform but this seems clearer
        let rect = [-radius, -radius, 2 * radius, 2 * radius];

        app::ellipse(fcolor, rect, trans, graphics);
    }
}




