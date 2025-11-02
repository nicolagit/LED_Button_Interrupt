//! LED control module
//!
//! This module provides functions to initialize and control an LED connected to a specified GPIO port and pin.

use crate::gpio::*;
use core::ptr;

unsafe fn read_register(addr: *mut u32) -> u32 {
    unsafe { ptr::read_volatile(addr) }
}

unsafe fn write_register(addr: *mut u32, value: u32) {
    unsafe { ptr::write_volatile(addr, value) }
}

fn clear_bits(value: u32, mask: u32) -> u32 {
    value & !mask
}

fn set_bits(value: u32, mask: u32) -> u32 {
    value | mask
}

/// Initializes the LED at the specified port and pin.
///
/// This function performs the following steps:
/// 1. Enables the GPIO port clock.
/// 2. Sets the GPIO pin mode to output mode.
/// 3. Sets the output type to push-pull.
/// 4. (Optional) Sets the output speed.
///
/// # Parameters
/// - `port`: The GPIO port where the LED is connected.
/// - `pin`: The GPIO pin number where the LED is connected.
/// # Warnings
/// # Note
/// # Example
/// ```
/// led_init(GPIOA_BASE, GPIO_PIN_0);
/// ```
pub fn led_init(port: u32, pin: u32) {
    //1. Enable the gpio port clock
    enable_gpio_clock(port);

    //2. Set the gpio pin mode = output mode
    set_gpio_mode_output(port, pin);

    //3. Set the output type = pushpull
    set_gpio_output_type_push_pull(port, pin);

    //4. Set the output speed (optional)
}

pub fn led_on(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::High);
}

pub fn led_off(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Low);
}

pub fn led_toggle(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Toggle);
}
