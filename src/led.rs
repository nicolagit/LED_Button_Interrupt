use core::ptr;
use crate::gpio::*;

unsafe fn read_register(addr: *mut u32) -> u32 {
    ptr::read_volatile(addr)
}

unsafe fn write_register(addr: *mut u32, value: u32) {
    ptr::write_volatile(addr, value)
}

fn clear_bits(value: u32, mask: u32) -> u32 {
    value & !mask
}

fn set_bits(value: u32, mask: u32) -> u32 {
    value | mask
}

pub fn led_init(port: u32, pin: u32) {
    //1. Set the gpio pin mode = output mode
    set_gpio_mode_output(port, pin);

    //2. Set the output type = pushpull
    set_gpio_output_type_push_pull(port, pin);

    //3. Set the output speed (optional)
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
