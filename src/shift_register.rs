use embedded_hal::digital::OutputPin;

pub struct ShiftRegister<DataPin, ClockPin, LatchPin> where 
    DataPin: OutputPin,
    ClockPin: OutputPin,
    LatchPin: OutputPin
{
    data_pin: DataPin,
    clock_pin: ClockPin,
    latch_pin: LatchPin
}
impl<DataPin, ClockPin, LatchPin> ShiftRegister<DataPin, ClockPin, LatchPin> where 
    DataPin: OutputPin,
    ClockPin: OutputPin,
    LatchPin: OutputPin
{
    pub fn new(data_pin: DataPin, clock_pin: ClockPin, latch_pin: LatchPin) -> Self{
        return ShiftRegister{ data_pin, clock_pin, latch_pin }
    }

    pub fn output(&mut self, size: u8, bits: u16){
        self.latch_pin.set_low().unwrap();
        for i in 0..size{            
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
}