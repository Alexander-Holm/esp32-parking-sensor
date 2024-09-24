use esp_hal::timer::Timer;
use fugit::{ Instant, MicrosDurationU64 };
use core::cmp::Ordering::{ Equal, Greater, Less };

pub struct SimpleTimer<'a>{
    timer: &'a dyn Timer,
    current_duration: Option< MicrosDurationU64 >,
    end_timestamp: Option< Instant<u64, 1, 1_000_000> >
}
impl<'a> SimpleTimer<'a> {
    pub fn new(timer: &'a dyn Timer) -> Self {
        return SimpleTimer { timer, current_duration: None, end_timestamp: None }
    }
    pub fn start(&mut self, duration: MicrosDurationU64){
        self.end_timestamp = Some(self.timer.now() + duration);
        self.current_duration = Some(duration);
    }
    pub fn update_duration(&mut self, new_duration: MicrosDurationU64){
        let duration_difference: MicrosDurationU64; 
        let adjusted_end: Instant<u64, 1, 1_000_000>;
        // Behöver inte hålla koll på overflows eftersom timestamps är u64
        match self.current_duration.unwrap().const_partial_cmp(new_duration).unwrap(){
            Equal => { return; }
            Greater => { // current > new
                duration_difference = self.current_duration.unwrap().checked_sub(new_duration).unwrap();
                adjusted_end = self.end_timestamp.unwrap().checked_sub_duration(duration_difference).unwrap();
            }
            Less => { // current < new
                duration_difference = new_duration.checked_sub(self.current_duration.unwrap()).unwrap();
                adjusted_end = self.end_timestamp.unwrap().checked_add_duration(duration_difference).unwrap();
            }
        }
        self.end_timestamp = Some(adjusted_end);
        self.current_duration = Some(new_duration);
    }
    pub fn is_done(&mut self) -> bool {
        if self.current_duration.is_none() || self.end_timestamp.is_none(){
            return true;
        }
        if self.timer.now() > self.end_timestamp.unwrap(){
            self.current_duration = None;
            self.end_timestamp = None;
            return true;
        }
        return false;
    }
    pub fn now(&self) -> Instant<u64, 1, 1_000_000> {
        return self.timer.now();
    }
}