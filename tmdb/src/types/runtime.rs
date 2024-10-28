use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Runtime {
    pub hours: u32,
    pub minutes: u32,
}

impl Runtime {
    pub fn new(hours: u32, minutes: u32) -> Self {
        Self { hours, minutes }
    }
}

impl From<String> for Runtime {
    fn from(value: String) -> Self {
        let minutes: u32 = value.split_whitespace().next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

        let hours = minutes / 60;
        let minutes = minutes % 60;

        Runtime::new(hours, minutes)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.hours, self.minutes) {
            (0, 0) => write!(f, "0 min"),              // special case if runtime is 0
            (0, m) => write!(f, "{} min", m),          // only minutes
            (h, 0) => write!(f, "{} hr", h),           // only hours
            (h, m) => write!(f, "{} hr {} min", h, m), // both hours and minutes
        }
    }
}
