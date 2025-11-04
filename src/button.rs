use crate::gpio;
use crate::exti;

pub enum ButtonStatus {
    Pressed,
    Released,
}

pub enum Trigger {
    RisingEdge,
    FallingEdge,
    BothEdges,
}

pub enum Mode {
    Input,
    Interrupt(Trigger),
}

pub fn button_init(port: u32, pin: u32, mode: Mode) {
    gpio::enable_gpio_clock(port);
    gpio::set_gpio_mode_input(port, pin);
    match mode {
        Mode::Interrupt(trigger) => {
            match trigger {
                Trigger::FallingEdge => {
                    // Configure the pin for falling edge detection
                    exti::gpio::set_edge(pin, exti::gpio::EdgeTrigger::Falling);
                },
                Trigger::RisingEdge => {
                    // Configure the pin for rising edge detection
                    exti::gpio::set_edge(pin, exti::gpio::EdgeTrigger::Rising);
                },
                Trigger::BothEdges => {
                    // do nothing
                },
            }
        }
        Mode::Input => {
            // do nothing
        }
    }
}

pub fn button_congure_interrupt() {}

pub fn button_read_status(_pin: i32) -> ButtonStatus {
    ButtonStatus::Released
}
