#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use led::*;
use button::*;

use crate::button::button_congure_interrupt;

mod startup_stm32f303;
mod mcu;
mod board;
mod led;
mod button;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    led_init(RED_LED);
    led_off(RED_LED);
    button_init(BUTTON_PIN);
    button_configure_interrupt(BUTTON_PIN);

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

// Button interrupt handler
fn EXTI0_Handler() {
    led_toggle(RED_LED);
}
