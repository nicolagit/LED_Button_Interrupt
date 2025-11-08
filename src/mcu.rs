pub const GPIOA_BASE: u32 = 0x4800_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x400; //0x4800_0400;
pub const GPIOC_BASE: u32 = GPIOA_BASE + 0x800; // 0x4800_0800

pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = 1;
pub const GPIO_PIN_2: u32 = 2;
pub const GPIO_PIN_3: u32 = 3; // Green LED LD1 is on PB3
pub const GPIO_PIN_13: u32 = 13; // User button B1 (PC13)

pub const RCC_BASE: u32 = 0x4002_1000;

pub const EXTI_BASE: u32 = 0x4002_0400;
