//#![crate_type = "rlib"]
#![no_std]
#![cfg_attr(feature = "libc", feature(libc))]
#[cfg(not(feature = "libc"))]
extern crate cty;
#[cfg(feature = "libc")]
extern crate libc as cty;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("bindings.rs");
