use embedded_hal::digital::OutputPin;

pub struct LedBar<
    DataPin: OutputPin,
    ClockPin: OutputPin,
    LatchPin: OutputPin
> 
{
    data_pin: DataPin,
    clock_pin: ClockPin,
    latch_pin: LatchPin,
    currently_lit_count: u8,
    number_of_leds: u8
}
impl<DataPin: OutputPin, ClockPin: OutputPin, LatchPin: OutputPin> LedBar<DataPin, ClockPin, LatchPin> {
    pub fn new(number_of_leds: u8, data_pin: DataPin, clock_pin: ClockPin, latch_pin: LatchPin) -> Self{
        return LedBar{ currently_lit_count: 0, number_of_leds, data_pin, clock_pin, latch_pin }
    }
    pub fn light_leds(&mut self, count: u8){
        self.currently_lit_count = count;
        let bits: u16 = (1 << count) - 1;
        self.latch_pin.set_low().unwrap();
        for i in 0..self.number_of_leds{            
            let bit = 1 << i;
            let is_set = bits & bit > 0;
            if is_set {
                self.data_pin.set_high().unwrap();
            } else{
                self.data_pin.set_low().unwrap();
            }
            self.clock_pin.set_high().unwrap();
            self.clock_pin.set_low().unwrap();
        }
        self.latch_pin.set_high().unwrap();
    }
    pub fn get_lit_count(&self) -> u8{
        return self.currently_lit_count;
    }
}