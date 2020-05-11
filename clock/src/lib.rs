use std::fmt;
const MINUTES_IN_DAY: i32 = 1440;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    fn wrap_minutes(mut minutes: i32) -> i32 {
        minutes %= MINUTES_IN_DAY;
        if minutes < 0 {
            minutes += MINUTES_IN_DAY;
        }
        minutes
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Clock::wrap_minutes(minutes + 60 * hours),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Clock::wrap_minutes(self.minutes + minutes),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
