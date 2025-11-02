#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use board::*;
use button::*;
use core::panic::PanicInfo;
use led::*;

//use crate::button::button_congure_interrupt;

mod board;
mod button;
mod gpio;
mod led;
mod mcu;
mod reg;
mod startup_stm32f303;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    led_init(GREEN_LED_PORT, GREEN_LED_PIN);
    led_on(GREEN_LED_PORT, GREEN_LED_PIN);
    led_off(GREEN_LED_PORT, GREEN_LED_PIN);

    button::button_init(
        USER_BTN_PORT,
        USER_BTN_PIN,
        Mode::Interrupt(Trigger::FallingEdge),
    );
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
