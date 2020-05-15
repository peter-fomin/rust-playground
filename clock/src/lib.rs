use std::fmt;

const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = MINUTES_IN_HOUR * 24;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { minutes: 0 }.set_minutes(hours * MINUTES_IN_HOUR + minutes)
    }

    pub fn set_minutes(mut self, minutes: i32) -> Self {
        self.minutes = (MINUTES_IN_DAY + minutes % MINUTES_IN_DAY) % MINUTES_IN_DAY;
        self
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        self.set_minutes(minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / MINUTES_IN_HOUR, self.minutes % MINUTES_IN_HOUR)
    }
}
