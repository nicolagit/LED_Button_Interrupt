enum ButtonStatus {
    Pressed,
    Released,
}

pub fn button_init(pin: i32) {

}

pub fn button_congure_interrupt() {

}

pub fn button_read_status(pin: i32) -> ButtonStatus {
    ButtonStatus::Released
}
