//! TODO
//!
//! ---

#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32f4;

use core::fmt::Write;
use cortex_m::asm;
use rt::ExceptionFrame;
use sh::hio;
use stm32f4::stm32f407;

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    let mut _peripherals = stm32f407::Peripherals::take().unwrap();

    loop {
        writeln!(stdout, "Hello, world!").unwrap();
        asm::bkpt();
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
