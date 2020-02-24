use std::fmt;

pub struct Clock {
    m: i32
}

const DAY_MINUTES: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = (hours*60 + minutes) % DAY_MINUTES;
        if m < 0 {
            m += DAY_MINUTES
        }
        Clock{m}
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.m + minutes)
    }
    pub fn hour(&self) -> i32 {
        self.m / 60
    }
    pub fn minute(&self) -> i32 {
        self.m % 60
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour(), self.minute())
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock {:0>2}:{:0>2}", self.hour(), self.minute())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}
