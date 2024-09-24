#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    gpio::{ Input, Io, Level, Output, Pull },
    ledc::{ self, timer::Timer, LSGlobalClkSource, Ledc, LowSpeed }, 
    peripherals::Peripherals, 
    prelude::*, 
    system::SystemControl, 
    timer::timg::TimerGroup,
};
use esp_println::logger;
mod led_bar;
use fugit::MicrosDurationU64;
use led_bar::LedBar;
mod ultrasonic_distance_sensor;
use ultrasonic_distance_sensor::{SensorState, UltrasonicDistanceSensor};
mod buzzer;
use buzzer::Buzzer;
mod simple_timer;
use simple_timer::SimpleTimer;


const MAX_DISTANCE:u64 = 200;
const LED_COUNT: u64 = 10;
const SENSOR_POLLING_RATE_MS: u64 = 100;
fn buzzer_off_interval(distance: u64) -> MicrosDurationU64 { (distance * 5).millis() }


#[entry]
fn main() -> ! {
    logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let pins = Io::new(peripherals.GPIO, peripherals.IO_MUX).pins;
    let timer = TimerGroup::new(peripherals.TIMG0, &clocks).timer0;
    
    let mut distance_sensor = UltrasonicDistanceSensor::new(
        MAX_DISTANCE,
        Output::new(pins.gpio1, Level::Low),
        Input::new(pins.gpio0, Pull::Down), 
        SimpleTimer::new(&timer),
        &clocks,
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
    let mut buzzer = Buzzer::new(
        500.Hz(),
        SimpleTimer::new(&timer), 
        pwm_channel, &mut pwm_timer
    );

    
    loop {
        match distance_sensor.read_distance() {            
            Ok(distance) => {
                distance_sensor.timer.start(SENSOR_POLLING_RATE_MS.millis());
                let lit_led_count = LED_COUNT - (distance / (MAX_DISTANCE / LED_COUNT));
                led_bar.light_leds(lit_led_count as u8);
                if !buzzer.is_on() && !buzzer.timer.is_done() {
                    buzzer.timer.update_duration(buzzer_off_interval(distance));
                }
            }
            Err(state) => match state {
                SensorState::NotStarted => {
                    if distance_sensor.timer.is_done(){
                        distance_sensor.start_measurement();
                    }
                }
                SensorState::AboveMaxDistance => {
                    distance_sensor.timer.start(SENSOR_POLLING_RATE_MS.millis());
                    if led_bar.get_lit_count() > 0 {
                        led_bar.light_leds(0);
                    }
                }
                SensorState::Measuring => {}
            }
        }

        if buzzer.timer.is_done(){
            if buzzer.is_on(){
                buzzer.set_off();
                if let Some(distance) = distance_sensor.last_reading() {
                    buzzer.timer.start(buzzer_off_interval(distance));
                }
            }
            else if distance_sensor.last_reading().is_some(){
                buzzer.set_on();
                buzzer.timer.start(60.millis());
            }
        }
    }    
}