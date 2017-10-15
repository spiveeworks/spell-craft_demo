extern crate ref_links;
pub mod physics;
use ref_links as links;
pub mod entities;
pub mod prelude {
    pub use links::{Owned, Link};
}

pub mod units;
pub mod events;

