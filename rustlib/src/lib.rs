#![no_std]

use core::panic::PanicInfo;

extern crate cty;

extern "C" {
    fn printf(fmt: *const cty::c_char, ...) -> cty::c_int;
}

#[no_mangle]
pub extern "C" fn add_two(a: i32) -> i32 {
    cprintf("add_two\n");
    panic!("p");
    a + 2
}

fn cprintf(s: &str) {
    unsafe {
        printf("%.*s\0".as_bytes().as_ptr(), s.len(), s.as_bytes().as_ptr());
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    cprintf("\npanic!\n");
    if let Some(loc) = panic_info.location() {
        let file = loc.file();
        let linenr = loc.line();
        unsafe {
            printf(
                "File: %.*s Line: %d\n\0".as_bytes().as_ptr(),
                file.len(),
                file.as_bytes().as_ptr(),
                linenr,
            );
        }
    }
    loop {}
}

/*
#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}
*/
