#![no_std]
#![no_main]

use core::arch::asm;
use core::cell::Cell;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use critical_section::Mutex;

use cortex_m_rt::entry;
use cortex_m_rt::{exception, ExceptionFrame};
use rp2040_hal::gpio::bank0::{Gpio0, Gpio12, Gpio1};
use rp2040_pac::{interrupt, Peripherals};

use rp2040_hal::gpio::{
    FunctionSioInput, FunctionSioOutput, Pin, Pins, PullUp, Interrupt
};
use rp2040_hal::Sio;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};

// TODO 1 - add the RP2040 bootloader
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/* Exercise 4 */
// TODO 13 - define a structure that store two pins:
//           - the switch_a pin
//           - the LED1 pin

// TODO 14 - define a sharable static variable that stores a type of the data structure

// TODO 15 - register an interrupt handler for IO_IRQ_BANK0
//           inside the critical section, toggle the pin and clear the interrupt
/***************/

#[entry]
fn main() -> ! {
    // TODO 2 - take the peripherals
    let mut peripherals = Peripherals::take().unwrap();

    // TODO 3 - enable the IO_BANK0 peripheral (lab 02 pac)
    let reset = &peripherals.RESETS;
    reset
    .reset()
    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while reset.reset_done().read().bits() & (1 << 5) == 0 {}

    // TODO 4 - initialize SIO with Sio::new()
    let sio = Sio::new(peripherals.SIO);

    // TODO 5 - initialize Pins with Pins::new()
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // TODO 6 - initialize GPIO for LED 2
    //          use Pin<GpioX, Function, PullType>
    //          with `pins.reconfigure()`
    let mut pin: Pin<Gpio0, FunctionSioOutput, PullUp> = pins.gpio0.reconfigure();

    /* Exercise 3 */

    // TODO 10 - trigger a hard fault
    /**************/
    // define an invalid memory address
    const INVALID_ADDRESS: *mut u32 = 0xf000_0000 as *mut u32;

    // write to it
    unsafe {
    // this triggers a hard fault
    write_volatile(INVALID_ADDRESS, 0);
}

    /* Exercise 4 */

    // TODO 16 - initialize the button
 
    // TODO 17 - enable the interrupt for the button for `Interrupt:EdgeLow`

    // TODO 18 - initialize the pin for LED2

    // TODO 18 - initialize the structure to store the led and the button

    // TODO 19 - store the local structure in the global variable

    // TODO 20 - unmask the NVIC IO_Bank0 interrupt
    /*************/

    let mut value = 1;
for _ in 0..1000 {
    value = 1 - value;
    // TODO 7 - write the value to the LED
    match value {
        1 => {
            if let Err(_) = pin.set_high() {
                panic!("Failed to set the pin high");
            }
        },
        _ => {
            if let Err(_) = pin.set_low() {
                panic!("Failed to set the pin low");
            }
        }
    }

    // TODO 8 - sleep
    for _ in 0..5000 {
        unsafe { 
            asm!("nop") 
        }
    }
}

// TODO 12 - delete the generated fault
//           use a for instead of a loop above
//           panic! here
panic!("An error occurred");

}

/* Exercise 3 */

// TODO 9 - register the hard fault handler
//          use the code from lab02 - bare metal to blink LED1


#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    // TODO 11 - move the hard fault handler code here and trigger
    //           a panic! in the hard fault handler
    const RESET: u32 = 0x4000_c000;
    const CLR: u32 = 0x3000;

    const RESET_DONE: u32 = 0x4000_c008;

    const GPIOX_CTRL: u32 = 0x4001_4004;
    const GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
    const GPIO_OUT_SET: *mut u32 = 0xd000_0014 as *mut u32;
    const GPIO_OUT_CLR: *mut u32 = 0xd000_0018 as *mut u32;

    // TODO - use fill in the GPIO number for LED2
    const LED: u32 = 1;

    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    // set the LED pin the SIO function in IO_BANK0
    let gpio_ctrl = (GPIOX_CTRL + 8 * LED) as *mut u32;
    unsafe {
        write_volatile(gpio_ctrl, 5);
    };

    // set the LED pin as output in SIO
    unsafe {
        write_volatile(GPIO_OE_SET, 1 << LED);
    };

    let mut value = 0;

    loop {
        value = 1 - value;
        // write the value to the LED
        let reg = match value {
            0 => GPIO_OUT_CLR,
            _ => GPIO_OUT_SET,
        };

        unsafe { write_volatile(reg, 1 << LED) }

        // sleep
        for _ in 0..5000 {
            unsafe { asm!("nop") }
        }
    }
    loop {}
}
