extern crate owned_rc;

use owned_rc as links;

pub mod prelude {
    pub use links::{Owned, Link};
}

pub mod units;
pub mod physics;
pub mod events;
pub mod entities;

