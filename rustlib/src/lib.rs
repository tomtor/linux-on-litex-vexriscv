#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;

use heapless::consts::*;
use heapless::String;

extern crate cty;

extern "C" {
    fn printf(fmt: *const cty::c_char, ...) -> cty::c_int;
}

extern "C" {
    fn write(fd: cty::c_int, p: *const cty::c_char, len: cty::size_t) -> cty::c_int;
}

#[no_mangle]
pub extern "C" fn add_two(a: i32) -> i32 {
    cprintf("add_two\n");
    // panic!("p");
    a + 2
}

fn cprintf(s: &str) {
    unsafe {
        printf("%.*s\0".as_bytes().as_ptr(), s.len(), s.as_bytes().as_ptr());
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut s: String<U128> = String::new();
    writeln!(s, "{}", panic_info).ok();
    unsafe {
        write(2, s.as_bytes().as_ptr(), s.len());
    }
    loop {}
}
