#![no_std]

extern crate libtransistor_sys;
use libtransistor_sys::*;

fn main() {
    let to_output = "Hello World";
    unsafe {
        svcOutputDebugString(to_output.as_ptr() as *mut _, to_output.len() as u64);
    }
}
