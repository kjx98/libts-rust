use crate::{Julian, TimeVal};
use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::os::raw::{c_char, c_int, c_long};

// seconds since Unix Epoch 1970-01-01 00:00:00 UTC
#[derive(Eq, Copy, Clone, Default)]
pub struct UnixTime(u64);

#[derive(Clone)]
pub struct Tz {
    offset: i64,
    tz_name: String,
}

#[cfg(target_os = "linux")]
extern "C" {
    static timezone: c_long;
    static daylight: c_int;
    static tzname: [*const c_char; 2];
    fn tzset();
}

lazy_static! {
    static ref LOCAL_TZ: Tz = {
        unsafe {
            tzset();
        }
        #[cfg(target_os = "linux")]
        let offset: i64 = unsafe { timezone };
        #[cfg(not(target_os = "linux"))]
        let offset: i64 = -28800;
        #[cfg(target_os = "linux")]
        let tz_name = unsafe {
            let tz_name = if daylight == 0 {
                CStr::from_ptr(tzname[0])
            } else {
                CStr::from_ptr(tzname[1])
            };
            tz_name.to_string_lossy().to_string()
        };
        #[cfg(not(target_os = "linux"))]
        let tz_name = "CST".to_string();
        Tz { offset, tz_name }
    };
}

pub struct Local {}

impl PartialEq for UnixTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for UnixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.date();
        if self.is_midnight() {
            write!(f, "{}-{:02}-{:02}UTC", y, m, d)
        } else {
            let (hh, mm, ss) = self.hms();
            write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}Z", y, m, d, hh, mm, ss)
        }
    }
}

impl PartialOrd for UnixTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnixTime {
    fn cmp(&self, b: &Self) -> Ordering {
        self.0.cmp(&b.0)
    }
}

impl From<Julian> for UnixTime {
    fn from(dt: Julian) -> UnixTime {
        UnixTime(dt.to_time_t())
    }
}

use crate::u64::{u64_add, u64_sub};
impl Add for UnixTime {
    type Output = UnixTime;
    fn add(self, rhs: UnixTime) -> UnixTime {
        let (sec, _) = u64_add(self.0, rhs.0);
        UnixTime(sec)
    }
}

impl AddAssign for UnixTime {
    fn add_assign(&mut self, rhs: UnixTime) {
        (self.0, _) = u64_add(self.0, rhs.0);
    }
}

impl Sub for UnixTime {
    type Output = UnixTime;
    fn sub(self, rhs: UnixTime) -> UnixTime {
        let (sec, _) = u64_sub(self.0, rhs.0);
        UnixTime(sec)
    }
}

impl SubAssign for UnixTime {
    fn sub_assign(&mut self, rhs: UnixTime) {
        (self.0, _) = u64_sub(self.0, rhs.0);
    }
}

impl UnixTime {
    pub const fn new(sec: u64) -> UnixTime {
        UnixTime(sec)
    }
    pub fn from_ymd(y: i32, m: i32, d: i32) -> UnixTime {
        let jdn = Julian::from((y, m, d));
        Self::from(jdn)
    }
    pub const fn from_hours(hr: u32) -> UnixTime {
        let sec: u64 = hr as u64 * 3600;
        UnixTime(sec)
    }
    pub fn and_hms(&self, hh: u32, mm: u32, ss: u32) -> UnixTime {
        let sec = hh * 3600 + mm * 60 + ss;
        UnixTime(self.0 + sec as u64)
    }
    pub fn and_nanos(&self, nano: u32) -> TimeVal {
        TimeVal::new(self.0, nano)
    }
    pub fn and_micros(&self, micros: u32) -> TimeVal {
        TimeVal::new(self.0, micros * 1000)
    }
    pub fn and_millis(&self, millis: u32) -> TimeVal {
        TimeVal::new(self.0, millis * 1_000_000)
    }
    pub fn date(&self) -> (i32, i32, i32) {
        Julian::from_time_t(self.0).date()
    }
    pub fn is_midnight(&self) -> bool {
        (self.0 % (3600 * 24)) == 0
    }
    pub fn hms(&self) -> (u32, u32, u32) {
        let hms = (self.0 % (3600 * 24)) as u32;
        let hh = hms / 3600;
        let mmss = hms % 3600;
        let mm = mmss / 60;
        let ss = mmss % 60;
        (hh, mm, ss)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
    pub fn now() -> UnixTime {
        use libc::{clock_gettime, timespec, CLOCK_REALTIME};
        use std::mem::MaybeUninit;
        let sec = unsafe {
            let mut tp = MaybeUninit::<timespec>::uninit().assume_init();
            clock_gettime(CLOCK_REALTIME, &mut tp);
            tp.tv_sec as u64
        };
        UnixTime(sec)
    }
}

impl Local {
    pub fn offset() -> i64 {
        LOCAL_TZ.offset
    }
    pub fn tzname() -> &'static str {
        &LOCAL_TZ.tz_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unixtime() {
        let ts1 = UnixTime::from(Julian::new(20220101).unwrap());
        assert_eq!(ts1.to_string(), "2022-01-01UTC");
        let (y, m, d) = ts1.date();
        assert_eq!(y, 2022);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
        let ts = UnixTime::now();
        println!("Current UnixTime: {}", ts);
    }

    #[test]
    fn test_local() {
        println!("Local: {}, offset: {}", Local::tzname(), Local::offset());
    }
}
