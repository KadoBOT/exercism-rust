use std::fmt;

#[derive(Eq, Debug, PartialEq)]
pub struct Clock(i32, i32);

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::get_time(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.0, self.1 + minutes)
    }

    fn get_hours(hours: i32) -> i32 {
        match hours / 24 {
            x if x >= 1 => hours % 24,
            _ if hours < 0 => Self::get_hours(24 + hours),
            _ => hours,
        }
    }

    fn get_time(hours: i32, minutes: i32) -> Self {
        match minutes / 60 {
            x if x >= 1 => Clock(Self::get_hours(x + hours), minutes % 60),
            _ if minutes < 0 => Self::get_time(hours - 1, 60 + minutes),
            _ => Clock(Self::get_hours(hours), minutes),
        }
    }
}
