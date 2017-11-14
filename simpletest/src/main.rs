#![feature(alloc)]
#![no_std]
extern crate libtransistor_sys;
extern crate alloc;

use libtransistor_sys::*;
use alloc::string::String;
use alloc::string::ToString;

fn main() {
    let mut to_output = "Hello world".to_string();
    unsafe {
        // TODO: This should be a const ptr no ?
        svcOutputDebugString(to_output.as_bytes_mut().as_mut_ptr(), to_output.len() as u64);
    }
}
