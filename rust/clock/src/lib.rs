use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
    let mut m = minutes;
    let mut h = hours;
    while m < 0 {
        h = h - 1;
        m = m + 60;
    }
    if m.abs() >= 60 {
        h = h + m / 60;
        m = (60 + m % 60) % 60;
    }

    if h.abs() >= 24 || h < 0 {
        h = (24 + h % 24) % 24;
    }
    (h, m)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = normalize(hours, minutes);
        Clock{ hours: h, minutes: m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = self.minutes + minutes;
        let h = self.hours + 0;
        let (h, m) = normalize(h, m);
        Clock{ hours: h, minutes: m }
    }
}