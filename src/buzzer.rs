use embedded_hal::pwm::SetDutyCycle;
use fugit::Rate;
use esp_hal::{
    gpio::OutputPin, ledc::{
        channel::{ self, config::PinConfig, Channel, ChannelIFace }, 
        timer::{ self, config::Duty, LSClockSource, Timer, TimerIFace }, 
        LowSpeed
    }, peripheral::Peripheral 
};


pub struct Buzzer<'a, Pin: OutputPin + Peripheral<P = Pin>>{
    channel: Channel<'a, LowSpeed, Pin>,
    pub is_on: bool
}
impl <'a, Pin: OutputPin + Peripheral<P = Pin>> Buzzer<'a, Pin>{
    pub fn new(
        frequency: Rate<u32, 1, 1>, 
        mut channel: Channel<'a, LowSpeed, Pin>,
        timer: &'a mut Timer<'a, LowSpeed>
    ) -> Self 
    {
        timer.configure(timer::config::Config {
            duty: Duty::Duty10Bit,
            clock_source: LSClockSource::APBClk,
            frequency: frequency
        }).unwrap();
        channel.configure(channel::config::Config {
            timer: timer,
            duty_pct: 0,
            pin_config: PinConfig::PushPull
        }).unwrap();
        return Self { channel, is_on: false }
    }
    pub fn set_on(&mut self){
        self.channel.set_duty_cycle_percent(50).unwrap();
        self.is_on = true;
    }
    pub fn set_off(&mut self){
        self.channel.set_duty_cycle_percent(0).unwrap();
        self.is_on = false;
    }
}