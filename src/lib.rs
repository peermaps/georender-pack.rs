#[macro_use]
extern crate lazy_static;

pub mod varint;

pub mod encode;
pub mod osm_types;

pub mod labels;
pub use labels::*;

mod node;
pub use node::*;

mod line;
pub use line::*;

mod area;
pub use area::*;

mod point;
pub use point::*;
