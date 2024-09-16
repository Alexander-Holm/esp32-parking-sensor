#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    gpio::{ Input, Io, Level, Output, Pull }, 
    ledc::{ self, timer::Timer, LSGlobalClkSource, Ledc, LowSpeed}, 
    peripherals::Peripherals, 
    prelude::*, 
    system::SystemControl, 
    timer::{timg::TimerGroup, PeriodicTimer}
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
    let pins = Io::new(peripherals.GPIO, peripherals.IO_MUX).pins;

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let timg1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let sensor_timer = timg0.timer0;
    let mut buzzer_timer = PeriodicTimer::new(timg1.timer0);
    
    let mut distance_sensor = UltrasonicDistanceSensor::new(
        Output::new(pins.gpio1, Level::Low),
        Input::new(pins.gpio0, Pull::Down), 
        &clocks, &sensor_timer
    );

    let mut shift_register = ShiftRegister::new(
        Output::new(pins.gpio7, Level::Low),
        Output::new(pins.gpio9, Level::Low),
        Output::new(pins.gpio8, Level::Low)
    );
    
    let mut pwm_controller = Ledc::new(peripherals.LEDC, &clocks);
    pwm_controller.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut pwm_timer: Timer<LowSpeed> = pwm_controller.get_timer(ledc::timer::Number::Timer0);
    let pwm_channel = pwm_controller.get_channel(ledc::channel::Number::Channel0, pins.gpio4);
    let mut buzzer = Buzzer::new(400.Hz(), pwm_channel, &mut pwm_timer);

    let mut current_distance: u64 = 0;
    // Måste startat timern för att första loopen ska köras korrekt
    buzzer_timer.start(0.micros()).unwrap();
    
    loop {
        if
        buzzer.is_on == false &&
        buzzer_timer.wait() == Ok(())
        {
            current_distance = distance_sensor.get_distance_cm();
            let led_bars = 10 - ( current_distance / 40 );
            let bits = (1 << led_bars) - 1;
            shift_register.output(10, bits);
            buzzer.set_on();
            buzzer_timer.start(100.millis()).unwrap();
        }

        else if 
        buzzer.is_on 
        && buzzer_timer.wait() == Ok(())
        {
            buzzer.set_off();
            let buzzer_off_interval = current_distance * 3;
            buzzer_timer.start(buzzer_off_interval.millis()).unwrap();
        }
    }    
}