#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    delay::Delay, 
    gpio::{ Input, Io, Level, Output, Pull }, 
    ledc::{ self, timer::Timer, LSGlobalClkSource, Ledc, LowSpeed}, 
    peripherals::Peripherals, 
    prelude::*, 
    system::SystemControl, 
    timer::timg::TimerGroup
};
use esp_println::logger;
mod shift_register;
use shift_register::ShiftRegister;
mod ultrasonic_distance_sensor;
use ultrasonic_distance_sensor::UltrasonicDistanceSensor;
mod buzzer;
use buzzer::Buzzer;

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
    
    let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut pwm_timer: Timer<LowSpeed> = ledc.get_timer(ledc::timer::Number::Timer0);
    let pwm_channel = ledc.get_channel(ledc::channel::Number::Channel0, pins.gpio4);
    let mut buzzer = Buzzer::new(200.Hz(), pwm_channel, &mut pwm_timer);


    loop {
        // Polling rate
        delay.delay_millis(1000);

        buzzer.toggle();

        let distance = ultrasonic_distance_sensor.get_distance_cm();
        let led_bars = 10 - ( distance / 40 );
        let bits = (1 << led_bars) - 1;
        shift_register.output(10, bits);
    }    
}