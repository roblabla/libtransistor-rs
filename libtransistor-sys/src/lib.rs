#![feature(lang_items, alloc_system, global_allocator, allocator_api)]
#![no_std]

extern crate cty;
extern crate alloc_system;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("bindings.rs");

#[global_allocator]
static ALLOCATOR: alloc_system::System = alloc_system::System;

#[lang = "start"]
unsafe fn start(main_ptr: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    let main : fn() = core::mem::transmute(main_ptr);
    main();
    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    // TODO: Crash the switch
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
