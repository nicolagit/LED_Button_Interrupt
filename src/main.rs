#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use crate::button::button_congure_interrupt;

mod startup_stm32f303;
mod led;
mod button;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    led_init(BLUE_LED);
    led_off(BLUE_LED);
    button_init(BUTTON_PIN);
    button_congure_interrupt(BUTTON_PIN);

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

// Button interrupt handler
fn EXTI0_Handler() {
    led_toggle(BLUE_LED);
}
