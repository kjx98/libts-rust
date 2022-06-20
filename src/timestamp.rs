use crate::Julian;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::{Duration, SystemTime};

const NANOS_PER_SEC: u32 = 1_000_000_000;

#[derive(Eq, Copy, Clone, Default)]
pub struct TimeVal {
    sec: u64,
    nano: u32,
}

#[derive(Default)]
pub struct SysClock {
    sim: bool,
    adj: TimeVal,
}

impl PartialEq for TimeVal {
    fn eq(&self, other: &Self) -> bool {
        self.sec == other.sec && self.nano == other.nano
    }
}

impl fmt::Display for TimeVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:09}", self.sec, self.nano)
    }
}

impl PartialOrd for TimeVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeVal {
    fn cmp(&self, b: &Self) -> Ordering {
        if self.sec == b.sec {
            self.nano.cmp(&b.nano)
        } else {
            self.sec.cmp(&b.sec)
        }
    }
}

impl From<Duration> for TimeVal {
    fn from(dur: Duration) -> TimeVal {
        TimeVal {
            sec: dur.as_secs() as u64,
            nano: dur.subsec_nanos(),
        }
    }
}

impl From<Julian> for TimeVal {
    fn from(dt: Julian) -> TimeVal {
        TimeVal {
            sec: dt.to_time_t(),
            nano: 0,
        }
    }
}

use crate::u64::{u64_add, u64_sub};
impl Add for TimeVal {
    type Output = TimeVal;
    fn add(self, rhs: TimeVal) -> TimeVal {
        let nano = self.nano + rhs.nano;
        let cc: u64 = if nano < NANOS_PER_SEC { 0 } else { 1 };
        let nano = if nano < NANOS_PER_SEC {
            nano
        } else {
            nano - NANOS_PER_SEC
        };
        let (sec, _) = u64_add(self.sec, rhs.sec + cc);
        TimeVal { sec, nano }
    }
}

impl AddAssign for TimeVal {
    fn add_assign(&mut self, rhs: TimeVal) {
        let nano = self.nano + rhs.nano;
        let cc: u64 = if nano < NANOS_PER_SEC { 0 } else { 1 };
        self.nano = if nano < NANOS_PER_SEC {
            nano
        } else {
            nano - NANOS_PER_SEC
        };
        (self.sec, _) = u64_add(self.sec, rhs.sec + cc);
    }
}

impl Sub for TimeVal {
    type Output = TimeVal;
    fn sub(self, rhs: TimeVal) -> TimeVal {
        let cc: u64 = if self.nano < rhs.nano { 1 } else { 0 };
        let nano = if self.nano < rhs.nano {
            NANOS_PER_SEC + self.nano - rhs.nano
        } else {
            self.nano - rhs.nano
        };
        let (sec, _) = u64_sub(self.sec, rhs.sec + cc);
        TimeVal { sec, nano }
    }
}

impl SubAssign for TimeVal {
    fn sub_assign(&mut self, rhs: TimeVal) {
        let cc: u64 = if self.nano < rhs.nano { 1 } else { 0 };
        self.nano = if self.nano < rhs.nano {
            NANOS_PER_SEC + self.nano - rhs.nano
        } else {
            self.nano - rhs.nano
        };
        (self.sec, _) = u64_sub(self.sec, rhs.sec + cc);
    }
}

impl TimeVal {
    pub const fn new(sec: u64, nano: u32) -> TimeVal {
        let sec = match sec.checked_add((nano / NANOS_PER_SEC) as u64) {
            Some(secs) => secs,
            None => panic!("overflow in TimeVal::new"),
        };
        let nano = nano % NANOS_PER_SEC;
        TimeVal { sec, nano }
    }
    pub const fn from_hours(hr: u32) -> TimeVal {
        let sec: u64 = hr as u64 * 3600;
        TimeVal { sec, nano: 0 }
    }
    pub fn and_nanos(&mut self, nano: u32) {
        self.nano = nano;
    }
    pub fn and_macros(&mut self, macros: u32) {
        self.nano = macros * 1000;
    }
    #[must_use]
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.sec as u64
    }
    /// Returns the fractional part of this `Duration`, in nanoseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by nanoseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one billion).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::libts::TimeVal;
    ///
    /// let duration = TimeVal::new(5, 10_000_000);
    /// assert_eq!(duration.as_secs(), 5);
    /// assert_eq!(duration.subsec_nanos(), 10_000_000);
    /// ```
    #[must_use]
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 {
        self.nano
    }
    pub fn as_secs_f64(&self) -> f64 {
        (self.sec as f64) + (self.nano as f64) / (NANOS_PER_SEC as f64)
    }
    pub fn to_duration(&self) -> Duration {
        Duration::new(self.sec as u64, self.nano)
    }
    pub fn date(&self) -> Julian {
        Julian::from_time_t(self.sec)
    }
}

impl SysClock {
    pub fn new(sim: bool) -> SysClock {
        SysClock {
            sim,
            ..Default::default()
        }
    }
    pub fn now(&self) -> TimeVal {
        let dur = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        if !self.sim {
            TimeVal::from(dur)
        } else {
            let ti = TimeVal::from(dur);
            ti + self.adj
        }
    }
    pub fn set_timeval(&mut self, tv: &TimeVal) {
        if !self.sim {
            panic!("Not Simulation Clock");
        }
        let adj = *tv - self.now();
        // no rollback time
        if self.adj < adj {
            self.adj = adj;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timestamp() {
        let ts1 = TimeVal::from(Julian::new(20220101).unwrap());
        let jd = ts1.date();
        let (y, m, d) = jd.date();
        assert_eq!(y, 2022);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_sysclock() {
        let clk = SysClock::new(false);
        let dt = clk.now().date();
        println!("Now: {}", dt);
    }
}
