use crate::Julian;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

pub enum Dur {
    Second = 1,
    Nano = 1_000_000_000,
    Micro = 1_000_000,
    Milli = 1_000,
}

const TS3_TIME_NANO: u32 = Dur::Nano as u32;
const SYS_JITTER: u64 = 100_000; // 100us

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
        let d = self.days();
        let (hh, mm, ss) = self.hms();
        if d > 0 {
            write!(
                f,
                "{} days {:02}:{:02}:{:02}.{:06}",
                d,
                hh,
                mm,
                ss,
                self.nano / 1000
            )
        } else if hh == 0 && mm == 0 && ss == 0 {
            if self.nano > 10_000 {
                write!(f, "{} us", self.nano / 1000)
            } else {
                write!(f, "{} ns", self.nano)
            }
        } else {
            write!(f, "{:02}:{:02}:{:02}.{:06}", hh, mm, ss, self.nano / 1000)
        }
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
        let sec = if nano < TS3_TIME_NANO {
            rhs.sec
        } else {
            rhs.sec + 1
        };
        let nano = if nano < TS3_TIME_NANO {
            nano
        } else {
            nano - Dur::Nano as u32
        };
        let (sec, _) = u64_add(self.sec, sec);
        TimeVal { sec, nano }
    }
}

impl AddAssign for TimeVal {
    fn add_assign(&mut self, rhs: TimeVal) {
        let nano = self.nano + rhs.nano;
        let sec = if nano < TS3_TIME_NANO {
            rhs.sec
        } else {
            rhs.sec + 1
        };
        self.nano = if nano < TS3_TIME_NANO {
            nano
        } else {
            nano - TS3_TIME_NANO
        };
        (self.sec, _) = u64_add(self.sec, sec);
    }
}

impl Add<u64> for TimeVal {
    type Output = TimeVal;
    fn add(self, rhs: u64) -> TimeVal {
        let sec = rhs / TS3_TIME_NANO as u64;
        let nano = (rhs % TS3_TIME_NANO as u64) as u32;
        let nano = self.nano + nano;
        let sec = if nano < Dur::Nano as u32 {
            sec
        } else {
            sec + 1
        };
        let nano = if nano < Dur::Nano as u32 {
            nano
        } else {
            nano - Dur::Nano as u32
        };
        let (sec, _) = u64_add(self.sec, sec);
        TimeVal { sec, nano }
    }
}

impl AddAssign<u64> for TimeVal {
    fn add_assign(&mut self, rhs: u64) {
        let sec = rhs / TS3_TIME_NANO as u64;
        let nano = (rhs % TS3_TIME_NANO as u64) as u32;
        let nano = self.nano + nano;
        let sec = if nano < TS3_TIME_NANO { sec } else { sec + 1 };
        self.nano = if nano < TS3_TIME_NANO {
            nano
        } else {
            nano - TS3_TIME_NANO
        };
        (self.sec, _) = u64_add(self.sec, sec);
    }
}

impl Sub for TimeVal {
    type Output = TimeVal;
    fn sub(self, rhs: TimeVal) -> TimeVal {
        let cc: u64 = if self.nano < rhs.nano { 1 } else { 0 };
        let nano = if self.nano < rhs.nano {
            TS3_TIME_NANO + self.nano - rhs.nano
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
            TS3_TIME_NANO + self.nano - rhs.nano
        } else {
            self.nano - rhs.nano
        };
        (self.sec, _) = u64_sub(self.sec, rhs.sec + cc);
    }
}

impl TimeVal {
    pub const fn new(sec: u64, nano: u32) -> TimeVal {
        let sec = match sec.checked_add((nano / TS3_TIME_NANO) as u64) {
            Some(secs) => secs,
            None => panic!("overflow in TimeVal::new"),
        };
        let nano = nano % TS3_TIME_NANO;
        TimeVal { sec, nano }
    }
    pub const fn from_hours(hr: u32) -> TimeVal {
        let sec: u64 = hr as u64 * 3600;
        TimeVal { sec, nano: 0 }
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
        (self.sec as f64) + (self.nano as f64) / (TS3_TIME_NANO as f64)
    }
    pub fn as_hours(&self) -> u32 {
        (self.sec / 3600) as u32
    }
    pub fn subhour_micros(&self) -> u32 {
        let secs = (self.sec % 3600) as u32;
        secs * 1_000_000 + (self.nano / 1000)
    }
    pub fn to_duration(&self) -> Duration {
        Duration::new(self.sec as u64, self.nano)
    }
    pub fn date(&self) -> (i32, i32, i32) {
        Julian::from_time_t(self.sec).date()
    }
    pub fn days(&self) -> u32 {
        (self.sec / (3600 * 24)) as u32
    }
    pub fn hms(&self) -> (u32, u32, u32) {
        let hms = (self.sec % (3600 * 24)) as u32;
        let hh = hms / 3600;
        let mmss = hms % 3600;
        let mm = mmss / 60;
        let ss = mmss % 60;
        (hh, mm, ss)
    }
    pub fn now() -> TimeVal {
        use libc::{clock_gettime, timespec, CLOCK_REALTIME};
        use std::mem::MaybeUninit;
        let (sec, nano) = unsafe {
            let mut tp = MaybeUninit::<timespec>::uninit().assume_init();
            clock_gettime(CLOCK_REALTIME, &mut tp);
            (tp.tv_sec as u64, tp.tv_nsec as u32)
        };
        TimeVal { sec, nano }
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
        use libc::{clock_gettime, timespec, CLOCK_REALTIME};
        use std::mem::MaybeUninit;
        /*
        let dur = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        */
        let dur = unsafe {
            let mut tp = MaybeUninit::<timespec>::uninit().assume_init();
            clock_gettime(CLOCK_REALTIME, &mut tp);
            let sec = tp.tv_sec as u64;
            let nano = tp.tv_nsec as u32;
            Duration::new(sec, nano)
        };
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

pub fn nsleep(nsec: u64) {
    use std::thread;
    let mut tv = TimeVal::now();
    let tpe = tv + nsec;
    if nsec > SYS_JITTER {
        thread::sleep(Duration::from_nanos(nsec));
        tv = TimeVal::now();
    }
    while tv < tpe {
        thread::yield_now();
        tv = TimeVal::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnixTime;
    use std::{thread::sleep, time::Duration};
    #[test]
    fn test_timestamp() {
        let ts1 = TimeVal::from(Julian::new(20220101).unwrap());
        let (y, m, d) = ts1.date();
        assert_eq!(y, 2022);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_sysclock() {
        let clk = SysClock::new(false);
        let ts = clk.now();
        println!("Now since epoch: {}", ts);
        let mut clk = SysClock::new(true);
        let ts1 = UnixTime::from_ymd(2022, 1, 1)
            .and_hms(8, 30, 0)
            .and_nanos(0);
        clk.set_timeval(&ts1);
        sleep(Duration::from_micros(100));
        let ts = clk.now() - ts1;
        println!("after sleep 2022-01-01: {}", ts);
    }

    #[test]
    fn test_nsleep() {
        let tv = TimeVal::now();
        nsleep(50_000);
        let tve = TimeVal::now();
        let ts = tve - tv;
        let dur = ts.to_duration();
        println!(
            "nsleep(50_000) cost ({}) {} hours {} us",
            ts,
            ts.as_hours(),
            ts.subhour_micros()
        );
        println!("nsleep(50_000) cost {} seconds", dur.as_secs_f64());
    }
}
