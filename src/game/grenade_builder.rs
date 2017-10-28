use std::mem;
use std::rc;

use charm_internal::forms::effects;
use charm_internal::forms::presets;
use charm_internal::units;

#[derive(Clone, Copy)]
enum Level {
    Low,
    Medium,
    High,
}

impl Level {
    fn choose<T>(self: Self, low: T, medium: T, high: T) -> T {
        match self {
            Level::Low => low,
            Level::Medium => medium,
            Level::High => high,
        }
    }
}

fn color_from_levels(settings: [Level; 3]) -> [u8; 3] {
    let   red = settings[0].choose(0x22, 0x99, 0xFF);
    let green = settings[0].choose(0x22, 0x99, 0xFF);
    let  blue = settings[0].choose(0x22, 0x99, 0xFF);

    [red, green, blue]
}



fn basic_grenade(settings: [Level; 4]) -> rc::Rc<effects::Cast> {
    let color_levels = [
        settings[0],
        settings[1],
        settings[2],
    ];
    let color = color_from_levels(color_levels);
    let radius = settings[3].choose(20, 120, 250) * units::DOT;

    presets::grenade(color, radius)
}

fn cluster_grenade(
    children: Vec<(units::Displacement, rc::Rc<effects::Cast>)>
) -> rc::Rc<effects::Cast> {
    presets::cluster_grenade(children.into_boxed_slice())
}


pub struct Builder {
    settings: [Level; 4],
    pub current: rc::Rc<effects::Cast>,
    available: [rc::Rc<effects::Cast>; 9],
    cluster_buffer: Vec<(units::Displacement, rc::Rc<effects::Cast>)>,
}


impl Builder {
    pub fn new() -> Self {
        let settings = [Level::Low; 4];
        let current = basic_grenade(settings);
        let available = [
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
            rc::Rc::clone(&current),
        ];
        let cluster_buffer = Vec::new();

        Builder { settings, current, available, cluster_buffer }
    }

    pub fn load(self: &mut Self, which: usize) {
        self.current = rc::Rc::clone(&self.available[which]);
    }

    pub fn save(self: &mut Self, which: usize) {
        self.available[which] = rc::Rc::clone(&self.current);
    }

    pub fn build_basic(self: &mut Self) {
        self.current = basic_grenade(self.settings);
    }

    pub fn add_to_cluster(self: &mut Self, offset: units::Displacement) {
        let new = rc::Rc::clone(&self.current);
        self.cluster_buffer.push((offset, new));
    }

    pub fn build_cluster(self: &mut Self) {
        let buffer = mem::replace(&mut self.cluster_buffer, Vec::new());
        self.current = cluster_grenade(buffer);
    }

    pub fn current(self: &Self) -> rc::Rc<effects::Cast> {
        rc::Rc::clone(&self.current)
    }
}
