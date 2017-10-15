extern crate ref_links;

use ref_links as links;

pub mod prelude {
    pub use links::{Owned, Link};
}

pub mod units;
pub mod events;

