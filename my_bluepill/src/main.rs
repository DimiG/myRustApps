//! ========================================================================
//! File: main.rs
//! Creator: Dmitri G.
//! Date: 2019-03-22
//! Version: 0.1.0
//! Description: This is a `Hello World` application written for BluePill
//! MCU STM32F103C8T6 LED PC13
//! ========================================================================

#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f1;

/* In case of semihosting usage uncomment below */
/* And comment `cortex_m_rt' */
//extern crate panic_semihosting;
//extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;

const MODE_OUTPUT_2MHZ: u8 = 0b10;
const CNF_OUTPUT_PUSHPULL: u8 = 0b00;

/* Use `main` as the entry point of this application */
/* The `main` is not allowed to return! */
#[entry]
fn main() -> ! {
    /* Initialization. In case of semihosting usage uncomment below */
    //hprintln!("Trying to HIGHLIGHT the LED PC13 for Blue Pill STM32F103C8T6! :)").unwrap();

    let peripherals = stm32f1::stm32f103::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_c = peripherals.GPIOC;

    rcc.apb2enr.write(|w| w.iopcen().bit(true));

    port_c.crh.write(|w| unsafe {
        w.mode13()
            .bits(MODE_OUTPUT_2MHZ)
            .cnf13()
            .bits(CNF_OUTPUT_PUSHPULL)
    });

    port_c.brr.write(|w| w.br13().set_bit());

    loop { /* Application Logic */ }
}
