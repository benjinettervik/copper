use std::time::Duration;
pub struct Time {
    pub delta_seconds: f32,
    pub elapsed_seconds: f32,
}

impl Time{
    pub fn new() -> Self{
        Self{
            delta_seconds:0.0,
            elapsed_seconds:0.0,
        }
    }

    pub fn register_delta(&mut self,delta:f32){
        self.delta_seconds = delta;
        self.elapsed_seconds += delta;
    }
    pub fn get_elapsed(self) ->f32{
        self.elapsed_seconds
    }
}