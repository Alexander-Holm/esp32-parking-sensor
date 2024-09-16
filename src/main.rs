#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::{ Input, Io, Level, Output, Pull }, ledc::{ self, timer::Timer, LSGlobalClkSource, Ledc, LowSpeed}, peripherals::Peripherals, prelude::*, system::SystemControl, timer::{timg::TimerGroup, PeriodicTimer}
};
use esp_println::logger;
mod led_bar;
use led_bar::LedBar;
mod ultrasonic_distance_sensor;
use ultrasonic_distance_sensor::UltrasonicDistanceSensor;
mod buzzer;
use buzzer::Buzzer;

const MAX_DISTANCE:u64 = 150;
const LED_COUNT: u64 = 10;


#[entry]
fn main() -> ! {
    logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
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

    let mut led_bar = LedBar::new(
        LED_COUNT as u8, 
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
        let buzzer_timer_is_done = buzzer_timer.wait() == Ok(());
        if
        !buzzer.is_on &&
        buzzer_timer_is_done
        {
            current_distance = distance_sensor.get_distance_cm();
            if current_distance < MAX_DISTANCE{
                let decimal_of_max = current_distance / (MAX_DISTANCE / LED_COUNT);
                let lit_led_count = LED_COUNT - decimal_of_max;
                led_bar.light_leds(lit_led_count as u8);
                buzzer.set_on();
                buzzer_timer.start(60.millis()).unwrap();
            }
            else { 
                if led_bar.get_lit_count() > 0{
                    led_bar.light_leds(0);
                }
                delay.delay_millis(100); 
            }
        }

        else if 
        buzzer.is_on &&
        buzzer_timer_is_done
        {
            buzzer.set_off();
            let buzzer_off_interval = current_distance * 5;
            buzzer_timer.start(buzzer_off_interval.millis()).unwrap();
        }
    }    
}