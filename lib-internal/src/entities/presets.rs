use std::rc;

use units;

use entities::effects;


struct BoltEffect;

impl effects::Effect for BoltEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0x00, 0x33, 0x00, 0xFF]
    }
}

struct SmokeEffect;

impl effects::Effect for SmokeEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0xFF, 0xCC, 0x00, 0xFF]
    }
}



pub fn grenade() -> rc::Rc<effects::Cast> {
    let smoke = effects::SmokeCast {
        shape: effects::Circle {
            color: rc::Rc::new(SmokeEffect),
            radius: 150 * units::DOT,
        },
        duration: 3 * units::MOMENT,
    };
    let bolt = effects::BoltCast {
        shape: effects::Circle {
            color: rc::Rc::new(BoltEffect),
            radius: 7 * units::DOT,
        },
        duration: 1 * units::SEC,
        action: rc::Rc::new(smoke),
    };
    rc::Rc::new(bolt)
}

