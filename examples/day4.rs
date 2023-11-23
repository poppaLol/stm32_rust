#![no_main]
#![no_std]
#![deny(unsafe_code)]

use cortex_m_semihosting::hprintln;
use panic_halt as _;

use stm32f3xx_hal as hal;

use cortex_m_rt::entry;
use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let switch1 = gpioe
        .pe4
        .into_input(&mut gpioe.moder);
    let switch2 = gpioe
        .pe5
        .into_input(&mut gpioe.moder);
    let switch3 = gpioe
        .pe6
        .into_input(&mut gpioe.moder);

    let mut led1 = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    (&mut gpioe.moder, &mut gpioe.otyper);
    let mut led2 = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led3 = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {

        match switch1.is_high() {
            Err(_e) => {},
            Ok(high) => match high {
                true => led1.set_high().unwrap(),
                false => led1.set_low().unwrap()
            }
        };
        match switch2.is_high() {
            Err(_e) => {},
            Ok(high) => match high {
                true => led2.set_high().unwrap(),
                false => led2.set_low().unwrap()
            }
        };
        match switch3.is_high() {
            Err(_e) => {},
            Ok(high) => match high {
                true => led3.set_high().unwrap(),
                false => led3.set_low().unwrap()
            }
        };
    }
}