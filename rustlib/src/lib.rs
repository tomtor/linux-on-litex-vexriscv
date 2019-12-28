#![no_std]

use core::panic::PanicInfo;

extern crate cty;

extern "C" {
        fn printf(fmt: *const cty::c_char, ...) -> cty::c_int;
}

#[no_mangle]
pub extern "C" fn add_two(a: i32) -> i32 {
    cprintf("add_two\n");
    // panic!("p");
    a + 2
}

fn cprintf(s: &str)
{
    unsafe {
      printf("%.*s\0".as_bytes().as_ptr(), s.len(), s.as_bytes().as_ptr());
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    cprintf("\npanic!\n");
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        cprintf(s);
        cprintf("\n");
    }
    // cprintf("\npanic2!\n");
    loop {
        // atomic::compiler_fence(Ordering::SeqCst);
    }
}

/*
#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}
*/
