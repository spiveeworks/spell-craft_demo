use std::rc;

use units;

use forms::effects;


struct BoltEffect;

impl effects::Effect for BoltEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0xFF, 0xFF, 0xFF, 0xFF]
    }
}

struct SmokeEffect {
    col: [u8; 3]
}

impl effects::Effect for SmokeEffect {
    fn color(self: &Self) -> [u8; 4] {
        [self.col[0], self.col[1], self.col[2], 0xFF]
    }
}


fn bolt(action: rc::Rc<effects::Cast>) -> rc::Rc<effects::Cast> {
    let color = rc::Rc::new(BoltEffect);
    let radius = 7 * units::DOT;
    let shape = effects::Circle { color, radius };

    let duration = 1 * units::SEC;

    let bolt_val = effects::BoltCast { shape, duration, action };
    rc::Rc::new(bolt_val)
}

pub fn grenade(
    smoke: [u8; 3],
    radius: units::Scalar,
) -> rc::Rc<effects::Cast> {
    let smoke_effect = SmokeEffect { col: smoke };
    let color = rc::Rc::new(smoke_effect);
    let shape = effects::Circle { color, radius };
    let duration = 3 * units::MOMENT;

    let smoke_val = effects::SmokeCast { shape, duration };
    let smoke = rc::Rc::new(smoke_val);

    bolt(smoke)
}


pub fn cluster_grenade(
    actions: Box<[(units::Displacement, rc::Rc<effects::Cast>)]>
) -> rc::Rc<effects::Cast> {
    let cluster_val = effects::ClusterCast { actions };
    let cluster = rc::Rc::new(cluster_val);

    bolt(cluster)
}

