#![no_main]
#![no_std]
#![deny(unsafe_code)]

use panic_halt as _;

//use stm32f3xx_hal as hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug,hprintln};
//use hal::pac;
//use hal::prelude::*;

#[entry]
fn main() -> ! {
    //this is taken from Rust Embedded book getting started template
    //as a hello world for reference
    //do setup here
    hprintln!("Hello, world!");

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {
        //do repeating work here
    }
}