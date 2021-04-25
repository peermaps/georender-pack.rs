#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

pub mod varint;

pub mod encode;
pub mod label;
pub mod osm_types;
pub mod tag_priorities;
pub mod tags;

mod node;
pub use node::*;

mod line;
pub use line::*;

mod area;
pub use area::*;

mod member;
pub use member::*;
