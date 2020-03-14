use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Clock {
    minutes: i32
}

const DAY_MINUTES: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{ minutes: (hours*60 + minutes).rem_euclid(DAY_MINUTES) }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
    fn hour(&self) -> i32 {
        self.minutes / 60
    }
    fn minute(&self) -> i32 {
        self.minutes % 60
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour(), self.minute())
    }
}
