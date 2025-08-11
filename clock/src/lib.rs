use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.adjust_clock()
    }

    fn get_hours_for_minutes(minutes: i32) -> i32 {
        let minutes_in_hour = 60;

        minutes.div_euclid(minutes_in_hour)
    }

    fn get_adjusted_minutes(minutes: i32) -> i32 {
        let max_minutes = 60;

        minutes.rem_euclid(max_minutes)
    }

    fn get_adjusted_hours(hours: i32) -> i32 {
        let max_hours = 24;

        hours.rem_euclid(max_hours)
    }

    fn adjust_clock(self) -> Self {
        let hours_to_add = Self::get_hours_for_minutes(self.minutes);

        Self {
            hours: Self::get_adjusted_hours(self.hours + hours_to_add),
            minutes: Self::get_adjusted_minutes(self.minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.adjust_clock()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
