#![feature(lang_items, alloc, alloc_system, global_allocator, allocator_api)]
#![no_std]

extern crate alloc;
extern crate alloc_system;
extern crate libtransistor_sys;

use libtransistor_sys::*;

// For now, we are the stdlib, so we provide the start method
#[lang = "start"]
unsafe fn start(main_ptr: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    let main : fn() = core::mem::transmute(main_ptr);
    main();
    0
}

// And the panic method
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(msg: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    use core::fmt::Write;

    // Print the panic message
    let mut s = alloc::string::String::new();
    let _ = s.write_fmt(msg);
    unsafe {
        svcOutputDebugString(s.as_bytes_mut().as_mut_ptr(), s.len() as u64);
    }
    // TODO: Crash the switch
    loop {}
}

// And the allocator
#[global_allocator]
static ALLOCATOR: alloc_system::System = alloc_system::System;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod sys {
    pub use libtransistor_sys::*;
}
