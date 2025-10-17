use core::ptr;

unsafe fn read_register(addr: *mut u32) -> U32 {
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
    let offset = 0;
    let gpio_mode_register_addr = (port + offset) as *mut u32;

    // Calculate the bit position for the given pin
    let bit_position = pin * 2;
    let mode_mask = 0x3 << bit_position;
    let mode_value = 0x1 << bit_position; // Output mode is '01'

    unsafe {
        let mut gpio_mode_register_value = read_register(gpio_mode_register_addr);
        gpio_mode_register_value = clear_bits(gpio_mode_register_value, mode_mask);
        gpio_mode_register_value = set_bits(gpio_mode_register_value, mode_value);
        write_register(gpio_mode_register_addr, gpio_mode_register_value);
    }

    //2. Set the output type = pushpull
    let offset = 0x04;
    let gpio_mode_register_addr = (port + offset) as *mut u32;

    let mode_mask = 0x1 << bit_position;
    let mode_value = 0x1 << bit_position; // Output mode is '01'

    unsafe {
        let mut gpio_mode_register_value = read_register(gpio_mode_register_addr);
        gpio_mode_register_value = clear_bits(gpio_mode_register_value, mode_mask);
        gpio_mode_register_value = set_bits(gpio_mode_register_value, mode_value);
        write_register(gpio_mode_register_addr, gpio_mode_register_value);
    }

    //3. Set the output speed (optional)
}

pub fn led_on(port: u32, pin: u32) {

}

pub fn led_off(port: u32, pin: u32) {

}

pub fn led_toggle(port: u32, pin: u32) {

}
