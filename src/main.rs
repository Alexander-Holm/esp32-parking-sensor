#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    delay::Delay, 
    gpio::{ Input, Io, Level, Output, Pull },
    peripherals::Peripherals, 
    prelude::*, 
    system::SystemControl, 
    timer::timg::TimerGroup,
};
use esp_println::logger;
mod shift_register;
use shift_register::ShiftRegister;
mod ultrasonic_distance_sensor;
use ultrasonic_distance_sensor::UltrasonicDistanceSensor;

#[entry]
fn main() -> ! {
    logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let timer = TimerGroup::new(peripherals.TIMG0, &clocks).timer0;
    let pins = Io::new(peripherals.GPIO, peripherals.IO_MUX).pins;
    
    let mut ultrasonic_distance_sensor = UltrasonicDistanceSensor::new(
        Output::new(pins.gpio1, Level::Low),
        Input::new(pins.gpio0, Pull::Down), 
        &clocks, &timer
    );

    let mut shift_register = ShiftRegister::new(
        Output::new(pins.gpio7, Level::Low),
        Output::new(pins.gpio9, Level::Low),
        Output::new(pins.gpio8, Level::Low)
    );

    loop {
        // Polling rate
        delay.delay_millis(1000);

        let distance = ultrasonic_distance_sensor.get_distance_cm();
        let led_bars = 10 - ( distance / 40 );
        let bits = (1 << led_bars) - 1;
        shift_register.output(10, bits);
    }
}