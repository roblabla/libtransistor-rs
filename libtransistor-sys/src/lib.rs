#![no_std]
extern crate cty;
pub use cty::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("bindings.rs");
