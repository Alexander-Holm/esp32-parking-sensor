use embedded_hal::digital::{ OutputPin, InputPin };
use esp_hal::clock::Clocks;
use esp_hal::delay::Delay;
use fugit::Instant;

use crate::simple_timer::SimpleTimer;

#[derive(PartialEq, Eq)]
pub enum SensorState {
    Measuring,
    NotStarted,
    AboveMaxDistance
}

pub struct UltrasonicDistanceSensor<'a, TrigPin: OutputPin, EchoPin: InputPin>{
    delay: Delay,
    trig: TrigPin,
    echo: EchoPin,
    echo_start: Option< Instant<u64, 1, 1_000_000> >,
    last_reading: Option<u64>,
    max_distance: u64,
    pub timer: SimpleTimer<'a>,
}

impl <'a, TrigPin: OutputPin, EchoPin: InputPin> UltrasonicDistanceSensor<'a, TrigPin, EchoPin>{
    pub fn new(max_distance: u64, trig: TrigPin, echo: EchoPin, timer: SimpleTimer<'a>, clocks: &Clocks) -> Self{
        let delay = Delay::new(&clocks);
        return UltrasonicDistanceSensor{ max_distance, trig, echo, timer, delay, echo_start: None, last_reading: None };
    }

    pub fn start_measurement(&mut self){
        self.trig.set_high().unwrap();
        self.delay.delay_micros(10);
        self.trig.set_low().unwrap();
        // Blocking, ok för det är väldigt kort tid
        while self.echo.is_low().unwrap() {}
        self.echo_start = Some(self.timer.now());
    }

    pub fn read_distance(&mut self) -> Result<u64, SensorState> {
        if self.echo_start == None{
            return Err(SensorState::NotStarted);
        }
        let now = self.timer.now();
        let elapsed_microseconds = now.checked_duration_since(self.echo_start.unwrap()).unwrap();
        let centimeters = elapsed_microseconds.to_micros() / 58;
        if self.echo.is_high().unwrap() {
            if centimeters > self.max_distance {
                self.last_reading = None;
                self.echo_start = None;
                return Err(SensorState::AboveMaxDistance);
            }
            else { 
                return Err(SensorState::Measuring);
            }
        }
        self.last_reading = Some(centimeters);
        self.echo_start = None;
        return Ok(centimeters);
    }

    pub fn last_reading(&self) -> Option<u64> {
        return self.last_reading;
    }
}