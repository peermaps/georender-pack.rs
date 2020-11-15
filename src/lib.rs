#[macro_use]
extern crate lazy_static;

pub mod encode;
pub mod osm_types;

pub mod tags;
pub use tags::*;

mod node;
pub use node::*;

mod line;
pub use line::*;

mod area;
pub use area::*;
