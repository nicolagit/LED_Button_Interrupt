use core::ptr;

unsafe fn read_register(addr: *mut u32) -> U32 {
    ptr::read_volatile(addr)
}

unsafe fn write_register(addr: *mut u32, value: u32) {
    ptr::write_volatile(addr, value)
}

pub fn led_init(port: u32, pin: u32) {
    //1. Set the gpio pin mode = output mode
    let offset = 0;
    let gpio_mode_register_addr = (port + offset) as *mut u32;
    unsafe {
        let gpio_mode_register_value = read_register(gpio_mode_register_addr);
    }

    //2. Set the output type = pushpull
    //3. Set the output speed (optional)
}

pub fn led_on(port: u32, pin: u32) {

}

pub fn led_off(port: u32, pin: u32) {

}

pub fn led_toggle(port: u32, pin: u32) {

}
