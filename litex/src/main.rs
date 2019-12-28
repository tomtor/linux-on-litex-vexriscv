#![no_std]
#![no_main]

use core::panic::PanicInfo;

use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
use core::str;

use heapless::consts::*;
use heapless::String;

use riscv::register::time;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    hprint("Hello World\n");

    for i in 1..10 {
        // hprint(& String::<U10>::from(i as u32)); hprint("\n");
        let mut s = String::<U32>::new();
        write!(&mut s, "i: {} 1/i: {}\n", i, 1.0 / i as f32).unwrap();
        hprint(&s);
        s.clear();
        write!(&mut s, "{}\n", time::read64()).unwrap();
        hprint(&s);
    }
    panic!("test");

    // loop {}
}

/* uart */
const CSR_UART_BASE: *mut u8 = (0xf0001000) as *mut u8;
const CSR_UART_TXFULL_ADDR: *mut u8 = (0xf0001004) as *mut u8;

fn uart_rxtx_write(value: u8) {
    unsafe {
        write_volatile(CSR_UART_BASE, value);
    }
}

fn uart_txfull_read() -> bool {
    unsafe { read_volatile(CSR_UART_TXFULL_ADDR) != 0 }
}

fn hprint(s: &str) {
    for c in s.bytes() {
        while uart_txfull_read() {}
        uart_rxtx_write(c);
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    hprint("\npanic!\n");
    let mut s = String::<U64>::new();
    write!(&mut s, "{}", panic_info).ok();
    hprint(&s);
    loop {
        // atomic::compiler_fence(Ordering::SeqCst);
    }
}
