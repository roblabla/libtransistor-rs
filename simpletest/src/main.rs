#![feature(alloc)]
#![no_std]

extern crate libtransistor;
#[macro_use]
extern crate alloc;

use libtransistor::sys::*;
use alloc::string::String;
use alloc::string::ToString;
use alloc::boxed::Box;

extern "C" fn thread_main(arg1: *mut libtransistor::sys::c_void) {
    let mut to_output = "Hello world from thread 2".to_string();
    unsafe {
        svcOutputDebugString(to_output.as_bytes_mut().as_mut_ptr(), to_output.len() as u64);
    }
}

macro_rules! handle_res {
    ( $result:expr ) => {
        if $result != 0 {
            let mut err = "Error".to_string();
            svcOutputDebugString(err.as_bytes_mut().as_mut_ptr(), err.len() as u64);
            //return;
        }
    }
}

fn main() {
    let mut to_output = "Hello world".to_string();
    let mut newstack = vec![0u8; 4096];
    //newstack = newstack.wrapping_offset(262143);
    unsafe {
        let mut thread = 0;
        svcOutputDebugString(to_output.as_bytes_mut().as_mut_ptr(), to_output.len() as u64);
        handle_res!(svcCreateThread(&mut thread, Some(thread_main), 0, newstack.as_mut_ptr() as _, 0, -2));
        core::mem::forget(newstack);
        handle_res!(svcStartThread(thread));

        // TODO: This should be a const ptr no ?
        svcOutputDebugString(to_output.as_bytes_mut().as_mut_ptr(), to_output.len() as u64);
        loop { svcSleepThread(10); }
    }
}
