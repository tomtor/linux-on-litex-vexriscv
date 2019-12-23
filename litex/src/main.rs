#![no_std]
#![no_main]

extern crate panic_halt;

use core::ptr::{read_volatile, write_volatile};
use core::str;
use core::fmt::Write;

use heapless::String;
use heapless::consts::*;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    hprint("Hello World\n");

    for i in 1..10 {
        // hprint(& String::<U10>::from(i as u32)); hprint("\n");
        let mut s = String::<U32>::new();
        write!(&mut s, "i: {} 1/i: {}\n", i, 1.0 / i as f64).unwrap();
        hprint(&s);
    }
    
    loop {}
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
