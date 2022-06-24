use crate::timestamp::Dur;
use crate::{Local, TimeVal, UnixTime};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

const TS3_TIME_SECOND: u32 = Dur::Second as u32;
const TS3_TIME_NANO: u32 = Dur::Nano as u32;
const TS3_TIME_MICRO: u32 = Dur::Micro as u32;
const TS3_TIME_MILLI: u32 = Dur::Milli as u32;

#[derive(Eq, Copy, Clone, Default)]
pub struct DateTime<const DUR: u32 = 1, const IS_UTC: bool = false> {
    time: i64,
}

impl<const DUR: u32, const IS_UTC: bool> PartialEq for DateTime<DUR, IS_UTC> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<const DUR: u32, const IS_UTC: bool> fmt::Display for DateTime<DUR, IS_UTC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sec = if DUR == 1 {
            self.time
        } else {
            self.time / DUR as i64
        };
        let subsec = if DUR == 1 { 0 } else { self.time % DUR as i64 };
        let sec = if IS_UTC {
            sec as u64
        } else {
            (sec - Local::offset()) as u64
        };
        let ut = UnixTime::new(sec);
        let (y, m, d) = ut.date();
        let (hh, mm, ss) = ut.hms();
        let dts = format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, hh, mm, ss);
        match DUR {
            TS3_TIME_SECOND => write!(f, "{}", dts),
            TS3_TIME_MICRO => write!(f, "{}.{:06}", dts, subsec),
            TS3_TIME_MILLI => write!(f, "{}.{:03}", dts, subsec),
            _ => panic!("NOT SUPPORT"),
        }
    }
}

impl<const DUR: u32, const IS_UTC: bool> From<TimeVal> for DateTime<DUR, IS_UTC> {
    fn from(tv: TimeVal) -> DateTime<DUR, IS_UTC> {
        let secs = if DUR == 1 {
            tv.as_secs() as i64
        } else {
            tv.as_secs() as i64 * DUR as i64
        };
        let subsec = if DUR == 1 {
            0
        } else {
            tv.subsec_nanos() / (TS3_TIME_NANO / DUR)
        };
        let time = secs + (subsec as i64);
        DateTime::<DUR, IS_UTC> { time }
    }
}

impl<const DUR: u32, const IS_UTC: bool> Add for DateTime<DUR, IS_UTC> {
    type Output = DateTime<DUR, IS_UTC>;
    fn add(self, rhs: DateTime<DUR, IS_UTC>) -> DateTime<DUR, IS_UTC> {
        let time = self.time + rhs.time;
        DateTime::<DUR, IS_UTC> { time }
    }
}

impl<const DUR: u32, const IS_UTC: bool> Add<u32> for DateTime<DUR, IS_UTC> {
    type Output = DateTime<DUR, IS_UTC>;
    fn add(self, rhs: u32) -> DateTime<DUR, IS_UTC> {
        let time = self.time + rhs as i64;
        DateTime::<DUR, IS_UTC> { time }
    }
}

impl<const DUR: u32, const IS_UTC: bool> AddAssign<u32> for DateTime<DUR, IS_UTC> {
    fn add_assign(&mut self, rhs: u32) {
        self.time += rhs as i64;
    }
}

impl<const DUR: u32, const IS_UTC: bool> Sub for DateTime<DUR, IS_UTC> {
    type Output = DateTime<DUR, IS_UTC>;
    fn sub(self, rhs: DateTime<DUR, IS_UTC>) -> DateTime<DUR, IS_UTC> {
        let time = self.time - rhs.time;
        DateTime::<DUR, IS_UTC> { time }
    }
}

impl<const DUR: u32, const IS_UTC: bool> Sub<u32> for DateTime<DUR, IS_UTC> {
    type Output = DateTime<DUR, IS_UTC>;
    fn sub(self, rhs: u32) -> DateTime<DUR, IS_UTC> {
        let time = self.time - rhs as i64;
        DateTime::<DUR, IS_UTC> { time }
    }
}

impl<const DUR: u32, const IS_UTC: bool> SubAssign<u32> for DateTime<DUR, IS_UTC> {
    fn sub_assign(&mut self, rhs: u32) {
        self.time -= rhs as i64;
    }
}

impl DateTime<TS3_TIME_MILLI, false> {
    pub const fn new_ms(time: i64) -> DateTime<TS3_TIME_MILLI, false> {
        DateTime::<TS3_TIME_MILLI>::new(time)
    }
}

impl DateTime<TS3_TIME_MICRO, false> {
    pub const fn new_us(time: i64) -> DateTime<TS3_TIME_MICRO, false> {
        DateTime::<TS3_TIME_MICRO>::new(time)
    }
}

impl<const DUR: u32, const IS_UTC: bool> DateTime<DUR, IS_UTC> {
    const fn new(time: i64) -> DateTime<DUR, IS_UTC> {
        assert!(DUR != 0);
        DateTime::<DUR, IS_UTC> { time }
    }
    #[must_use]
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        let sec = self.time / DUR as i64;
        sec as u64
    }
    pub fn as_secs_f64(&self) -> f64 {
        (self.time as f64) / (DUR as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_datetime() {
        let dt = DateTime::<TS3_TIME_MILLI>::new(8123);
        assert_eq!(dt.as_secs(), 8);
        assert_eq!(dt.as_secs_f64(), 8.123);
        let dt = DateTime::<TS3_TIME_MICRO, true>::new(8123456);
        assert_eq!(dt.as_secs(), 8);
        assert_eq!(dt.as_secs_f64(), 8.123456);
        println!("DateTime<us>: {}", dt);
        let dt = dt + 111000;
        assert_eq!(dt.as_secs_f64(), 8.234456);
        let dt = DateTime::<1>::new(8123);
        assert_eq!(dt.as_secs(), 8123);
        assert_eq!(dt.as_secs_f64(), 8123.0);
        let dt = DateTime::new_us(8123456);
        assert_eq!(dt.as_secs(), 8);
        assert_eq!(dt.as_secs_f64(), 8.123456);
    }
}
