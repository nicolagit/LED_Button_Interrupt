#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

use core::panic::PanicInfo;
use led::*;
use button::*;
use board::*;

use crate::button::button_congure_interrupt;

mod startup_stm32f303;
mod mcu;
mod board;
mod led;
mod button;
mod gpio;
mod reg;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    led_init(GREEN_LED_PORT, GREEN_LED_PIN);
    led_on(GREEN_LED_PORT, GREEN_LED_PIN);
    //button_init(BUTTON_PIN);
    //button_configure_interrupt(BUTTON_PIN);

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

// Button interrupt handler
#[allow(non_snake_case)]
fn EXTI0_Handler() {
    led_toggle(GREEN_LED_PORT, GREEN_LED_PIN);
}
