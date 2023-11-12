#![no_main]
#![no_std]
#![deny(unsafe_code)]

use panic_halt as _;

use stm32f3xx_hal as hal;

use cortex_m_rt::entry;
use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOE.split(&mut rcc.ahb);

    let switch = gpioa
        .pe0
        .into_input(&mut gpioa.moder);

    let mut led = gpioa
        .pe1
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    led.set_low().unwrap();

    loop {
        //led.toggle().unwrap();
        match switch.is_high() {
            Err(_e) => {},
            Ok(high) => match high {
                true => led.set_high().unwrap(),
                false => led.set_low().unwrap()
            }
        };
    }
}