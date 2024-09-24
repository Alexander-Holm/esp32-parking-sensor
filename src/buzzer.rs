use embedded_hal::pwm::SetDutyCycle;
use fugit::Rate;
use esp_hal::{
    gpio::OutputPin, 
    ledc::{
        channel::{ self, config::PinConfig, Channel, ChannelIFace }, 
        timer::{ self, config::Duty, LSClockSource, Timer, TimerIFace }, 
        LowSpeed
    }, 
    peripheral::Peripheral,
};

use crate::simple_timer::SimpleTimer;


pub struct Buzzer<'a, Pin: OutputPin + Peripheral<P = Pin>>{
    pwm_channel: Channel<'a, LowSpeed, Pin>,
    is_on: bool,
    pub timer: SimpleTimer<'a>,
}
impl <'a, Pin: OutputPin + Peripheral<P = Pin>> Buzzer<'a, Pin>{
    pub fn new(
        frequency: Rate<u32, 1, 1>, 
        timer: SimpleTimer<'a>,
        mut pwm_channel: Channel<'a, LowSpeed, Pin>,
        pwm_timer: &'a mut Timer<'a, LowSpeed>,
    ) -> Self 
    {
        pwm_timer.configure(timer::config::Config {
            duty: Duty::Duty10Bit,
            clock_source: LSClockSource::APBClk,
            frequency: frequency
        }).unwrap();
        pwm_channel.configure(channel::config::Config {
            timer: pwm_timer,
            duty_pct: 0,
            pin_config: PinConfig::PushPull
        }).unwrap();
        return Self { pwm_channel, is_on: false, timer }
    }
    pub fn set_on(&mut self){
        self.pwm_channel.set_duty_cycle_percent(50).unwrap();
        self.is_on = true;
    }
    pub fn set_off(&mut self){
        self.pwm_channel.set_duty_cycle_percent(0).unwrap();
        self.is_on = false;
    }
    pub fn is_on(&self) -> bool {
        return self.is_on;
    }
}