use std::fmt;
use std::time::Duration;

const NANOS_PER_SEC: u32 = 1_000_000_000;

#[derive(Eq, Copy, Clone, Default)]
pub struct TimeVal {
    sec: u64,
    nano: u32,
}

#[derive(Clone, Default)]
pub struct SysClock {
    sim: bool,
    adj_sec: u64,
    adj_nano: u32,
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

impl From<Duration> for TimeVal {
    fn from(dur: Duration) -> TimeVal {
        TimeVal {
            sec: dur.as_secs() as u64,
            nano: dur.subsec_nanos(),
        }
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
}
