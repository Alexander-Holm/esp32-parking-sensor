use embedded_hal::digital::{ OutputPin, InputPin };
use esp_hal::clock::Clocks;
use esp_hal::delay::Delay;
use esp_hal::timer::Timer as TimerTrait;

pub struct UltrasonicDistanceSensor<'lifetime, TrigPin: OutputPin, EchoPin: InputPin>{
    trig: TrigPin,
    echo: EchoPin,
    timer: &'lifetime dyn TimerTrait,
    delay: Delay
}
impl <'lifetime, TrigPin: OutputPin, EchoPin: InputPin> UltrasonicDistanceSensor<'lifetime, TrigPin, EchoPin>{
    pub fn new(trig: TrigPin, echo: EchoPin, clocks: &Clocks, timer: &'lifetime dyn TimerTrait) -> Self{
        let delay = Delay::new(&clocks);
        return UltrasonicDistanceSensor{ trig, echo, timer, delay};
    }
    pub fn get_distance_cm(&mut self) -> u64{
        self.start_measurement();
        let microseconds = self.count_echo_length();
        let centimeters = self.calculate_distance_cm(microseconds);
        return centimeters;

    }

    // Private methods

    fn start_measurement(&mut self){
        self.trig.set_high().unwrap();
        self.delay.delay_micros(10);
        self.trig.set_low().unwrap();
    }    
    fn count_echo_length(&mut self) -> u64 {
        // Vänta på att echo blir HIGH.
        // Skulle kunna ha litet delay i loopen för att spara energi
        while self.echo.is_low().unwrap() {}
        // echo är HIGH så starta timern
        let start_timestamp = self.timer.now();
        while self.echo.is_high().unwrap() {}
        let end_timestamp = self.timer.now();    
        let elapsed_duration = end_timestamp.checked_duration_since(start_timestamp).unwrap();
        return elapsed_duration.to_micros();
    }    
    fn calculate_distance_cm(&self, microseconds: u64) -> u64 {
        // Blir något längre avstånd än vad som väljs i Wokwi simulatorn.
        // Andra projekt jag hittar har samma problem.
        // T.ex: 400 -> 405,    300 -> 304,     200 -> 202,     100 -> 101
        return microseconds / 58;
    }
}