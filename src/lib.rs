#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

pub mod varint;

pub mod element;
pub mod encode;
pub mod label;
pub mod osm_types;
pub mod tags;

mod node;
pub use node::*;

mod line;
pub use line::*;

mod area;
pub use area::*;

mod point;
pub use point::*;
