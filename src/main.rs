#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    delay::Delay, 
    gpio::{ GpioPin, Input, Io, Level, Output, Pull }, 
    peripherals::{Peripherals, TIMG0}, 
    prelude::*, 
    system::SystemControl, 
    timer::{self, timg::{ TimerGroup, Timer0 }}, 
    Blocking
};
use esp_println::{ println, logger };

type TrigPin<'a> = Output<'a, GpioPin<1>>;
type EchoPin<'a> = Input<'a, GpioPin<0>>;
type Timer = timer::timg::Timer<Timer0<TIMG0>, Blocking>;

#[entry]
fn main() -> ! {
    logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let timer: Timer = TimerGroup::new(peripherals.TIMG0, &clocks).timer0;

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    // Variabler döpta efter namn på pins hos HC-SR04 Ultrasonic Distance Sensor
    let mut trig: TrigPin = Output::new(io.pins.gpio1, Level::Low);
    let echo:EchoPin = Input::new(io.pins.gpio0, Pull::Down);

    loop {
        // Polling rate
        delay.delay_millis(1000);

        start_measurement(&mut trig, delay);
        let time = count_echo_length(&echo, &timer);
        let distance = calculate_distance_cm(time);
        println!("Elapsed time: {time}");
        println!("Distance: {distance}");        
    }
}

fn start_measurement(trig_pin: &mut TrigPin, delay: Delay){
    trig_pin.set_high();
    delay.delay_micros(10);
    trig_pin.set_low();
}

fn count_echo_length(echo_pin: &EchoPin, timer: &Timer) -> u64 {
    // Vänta på att echo blir HIGH.
    // Skulle kunna ha litet delay i loopen för att spara energi
    while echo_pin.is_low() {}
    // echo är HIGH så starta timern
    let start_timestamp = timer.now();
    while echo_pin.is_high() {}
    let end_timestamp = timer.now();    
    let elapsed_duration = end_timestamp.checked_duration_since(start_timestamp).unwrap();
    return elapsed_duration.to_micros();
}

fn calculate_distance_cm(microseconds: u64) -> u64 {
    // Blir något längre avstånd än vad som väljs i Wokwi simulatorn.
    // Andra projekt jag hittar har samma problem.
    // T.ex: 400 -> 405,    300 -> 304,     200 -> 202,     100 -> 101
    return microseconds / 58;
}