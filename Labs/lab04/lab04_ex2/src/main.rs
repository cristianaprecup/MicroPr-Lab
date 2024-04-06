#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use embassy_executor::Spawner;

// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// Timer
use embassy_time::Timer;
use embassy_time::Duration;

// The following example turns on a led at 50% intensity.
// The led in this example is connected to GP0.
// ---- Exercise 2 - part 1 ----
// TODO 1: Modify the code to make the LED in your circuit light up at 25% intensity.
// -----------------------------
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals
    let peripherals = embassy_rp::init(Default::default());

    // Create config for PWM
    let mut config: PwmConfig = Default::default();
    config.top = 0x1000;
    config.compare_a = config.top / 6;

    // Initialize PWM
    let  mut _pwm = Pwm::new_output_a(peripherals.PWM_CH0, peripherals.PIN_0, config.clone());

    loop {
        // delete this otherwise it will panic
        // todo!();
        // ---- Exercise 2 - part 2 ----
        // TODO 2: Wait a second (Timer)
        Timer::after_secs(1).await;
        // TODO 3: Increment duty cycle of the led
        config.compare_a = config.compare_a + 100;
        // TODO 4: Modify the PWM configuration to use the new duty cycle
        _pwm.set_config(&config);
        // TODO 5: Check if it reached max PWM; if yes, don't increment anymore
        if config.compare_a >= config.top {
            config.compare_a = config.top;
        }
        // -----------------------------
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
