use prelude::*;

use units;
use physics;
use events;

use super::*;

struct BoltEffect;

impl Effect for BoltEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0x00, 0x33, 0x00, 0xFF]
    }
}

struct SmokeEffect;

impl Effect for SmokeEffect {
    fn color(self: &Self) -> [u8; 4] {
        [0xFF, 0xCC, 0x00, 0xFF]
    }
}



fn grenade() -> Rc<Cast> {
    let smoke = SmokeCast {
        shape: Circle {
            color: rc::Rc::new(SmokeEffect),
            radius: 150 * units::DOT,
        },
        duration: 3 * units::MOMENT,
    };
    let bolt = BoltCast {
        shape: Circle {
            color: rc::Rc::new(BoltEffect),
            radius: 7 * units::DOT,
        }
        duration: 1 * units::SEC,
        action: rc::Rc::new(smoke),
    };
    rc::Rc::new(bolt)
}

