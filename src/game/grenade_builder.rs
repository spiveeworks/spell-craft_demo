use std::mem;
use std::rc;

use charm_internal::forms::effects;
use charm_internal::forms::presets;
use charm_internal::units;

#[derive(Clone, Copy)]
pub enum Level {
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
    let green = settings[1].choose(0x22, 0x99, 0xFF);
    let  blue = settings[2].choose(0x22, 0x99, 0xFF);

    [red, green, blue]
}



fn basic_grenade(settings: [Level; 4]) -> rc::Rc<effects::Cast> {
    let color_levels = [
        settings[0],
        settings[1],
        settings[2],
    ];
    let color = color_from_levels(color_levels);
    let radius = settings[3].choose(20, 80, 120) * units::DOT;

    presets::grenade(color, radius)
}

fn cluster_grenade(
    children: Vec<(units::Displacement, rc::Rc<effects::Cast>)>
) -> rc::Rc<effects::Cast> {
    presets::cluster_grenade(children.into_boxed_slice())
}

enum IndexEnum {
    Register(usize),
    Dangle(rc::Rc<effects::Cast>),
}

pub struct Builder {
    settings: [Level; 4],
    current: IndexEnum,
    available: [rc::Rc<effects::Cast>; 10],
    cluster_buffer: Vec<(units::Displacement, rc::Rc<effects::Cast>)>,
}

pub enum ArsenalUpdate {
    SetLevel {
        which: usize,
        level: Level,
    },
    LoadNade {
        which: usize,
    },
    SaveNade {
        which: usize,
    },
    BuildCluster,
}

impl Builder {
    pub fn new() -> Self {
        let settings = [Level::Low; 4];

        let current = IndexEnum::Register(0);

        let def_nade = basic_grenade(settings);
        let available = [
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
            rc::Rc::clone(&def_nade),
        ];
        let cluster_buffer = Vec::new();

        Builder { settings, current, available, cluster_buffer }
    }

    pub fn apply_update(self: &mut Self, upd: ArsenalUpdate) {
        use self::ArsenalUpdate::*;
        match upd {
            SetLevel { which, level } => {
                self.settings[which] = level;
                self.build_basic();
            },
            LoadNade { which } => {
                self.load(which);
            },
            SaveNade { which } => {
                self.save(which);
            },
            BuildCluster => {
                self.build_cluster();
            },
        }
    }

    fn load(self: &mut Self, which: usize) {
        self.current = IndexEnum::Register(which);
    }

    fn save(self: &mut Self, which: usize) {
        if which > 0 {
            self.available[which] = self.current();
        }
    }

    fn build_basic(self: &mut Self) {
        self.available[0] = basic_grenade(self.settings);
    }

    pub fn add_to_cluster(self: &mut Self, offset: units::Displacement) {
        let new = self.current();
        self.cluster_buffer.push((offset, new));
    }

    fn build_cluster(self: &mut Self) {
        if self.cluster_buffer.len() > 0 {
            let buffer = mem::replace(&mut self.cluster_buffer, Vec::new());
            let nade = cluster_grenade(buffer);
            self.current = IndexEnum::Dangle(nade);
        }
    }

    pub fn current(self: &Self) -> rc::Rc<effects::Cast> {
        use self::IndexEnum::*;
        match self.current {
            Register(which) => rc::Rc::clone(&self.available[which]),
            Dangle(ref nade) => rc::Rc::clone(nade),
        }
    }
}

