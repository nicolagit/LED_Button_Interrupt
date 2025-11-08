use crate::mcu::*;
use crate::reg::*;

pub mod gpio
{
    use super::*; // Import everything from parent

    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub  fn set_edge(pin: u32, edge: EdgeTrigger) {
        let exti_rtsr1_addr = (EXTI_BASE + 0x08) as *mut u32;
        let exti_ftsr1_addr = (EXTI_BASE + 0x0C) as *mut u32;

        match edge {
            EdgeTrigger::Rising => {
                reg_set_bit(exti_ftsr1_addr, pin, true);

            }
            EdgeTrigger::Falling => {
                reg_set_bit(exti_rtsr1_addr, pin, true);
            }
        }
    }
}

pub enum ExtiLine {
    Line0 = 0,
    Line1 = 1,
    Line2 = 2,
    Line3 = 3,
    Line4 = 4,
    Line5 = 5,
    Line6 = 6,
    Line7 = 7,
    Line8 = 8,
    Line9 = 9,
    Line10 = 10,
    Line11 = 11,
    Line12 = 12,
    Line13 = 13,
    Line14 = 14,
    Line15 = 15,
}

impl ExtiLine {
    pub fn from_pin(pin: u32) -> Option<ExtiLine> {
        match pin {
            0 => Some(ExtiLine::Line0),
            1 => Some(ExtiLine::Line1),
            2 => Some(ExtiLine::Line2),
            3 => Some(ExtiLine::Line3),
            4 => Some(ExtiLine::Line4),
            5 => Some(ExtiLine::Line5),
            6 => Some(ExtiLine::Line6),
            7 => Some(ExtiLine::Line7),
            8 => Some(ExtiLine::Line8),
            9 => Some(ExtiLine::Line9),
            10 => Some(ExtiLine::Line10),
            11 => Some(ExtiLine::Line11),
            12 => Some(ExtiLine::Line12),
            13 => Some(ExtiLine::Line13),
            14 => Some(ExtiLine::Line14),
            15 => Some(ExtiLine::Line15),
            _ => None,
        }
    }
}

pub fn enable_interrupt(exti_line: ExtiLine) {
    // TODO
}

pub fn disable_interrupt(pin: u32) {
    // TODO
}
