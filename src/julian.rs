use std::fmt;

#[derive(Eq, Clone)]
pub struct Julian(i32);

// Julian Date for 1970-01-01
const TS3_JULIAN_EPOCH: i32 = 2440588;

impl Default for Julian {
    fn default() -> Self {
        Julian(TS3_JULIAN_EPOCH)
    }
}

impl PartialEq for Julian {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for Julian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.date();
        write!(f, "{}-{:02}-{:02}", y, m, d)
    }
}

impl From<i16> for Julian {
    fn from(da: i16) -> Julian {
        Julian(da as i32)
    }
}

impl From<i32> for Julian {
    fn from(da: i32) -> Julian {
        Julian(da)
    }
}

impl From<u16> for Julian {
    fn from(da: u16) -> Julian {
        Julian(da as i32)
    }
}

impl From<u32> for Julian {
    fn from(da: u32) -> Julian {
        Julian(da as i32)
    }
}

impl From<(i32, i32, i32)> for Julian {
    fn from(da: (i32, i32, i32)) -> Julian {
        Julian::new_jdn(da.0, da.1, da.2).unwrap()
    }
}

impl Julian {
    pub fn new(date: i32) -> Option<Julian> {
        let yr = date / 10000;
        let mon = (date % 1000) / 100;
        let mday = date % 100;
        Julian::new_jdn(yr, mon, mday)
    }
    fn new_jdn(yr: i32, mon: i32, mday: i32) -> Option<Julian> {
        if mon < 1 || mon > 12 || mday < 1 || mday > 31 {
            return None;
        }
        let res = (1461 * (yr + 4800 + (mon - 14) / 12)) / 4;
        let res = res + (367 * (mon - 2 - 12 * ((mon - 14) / 12))) / 12;
        let res = res - (3 * ((yr + 4900 + (mon - 14) / 12) / 100)) / 4;
        let res = res + mday - 32075;
        Some(Julian(res))
    }
    pub fn date(&self) -> (i32, i32, i32) {
        let f = self.0 + 1401 + (((4 * self.0 + 274277) / 146097) * 3) / 4 - 38;
        let e = 4 * f + 3;
        let g = (e % 1461) / 4;
        let h = 5 * g + 2;
        let d = (h % 153) / 5 + 1;
        let m = (h / 153 + 2) % 12 + 1;
        let y = e / 1461 - 4716 + (12 + 2 - m) / 12;
        (y, m, d)
    }
    // return julian date number
    pub fn jdn(&self) -> i32 {
        self.0
    }
    pub fn to_time_t(&self) -> u64 {
        debug_assert!(self.0 >= TS3_JULIAN_EPOCH);
        let res: u64 = (self.0 - TS3_JULIAN_EPOCH) as u64;
        let res = res * 3600 * 24;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch() {
        let jd: Julian = Default::default();
        let epo = Julian::new(19700101).unwrap();
        assert!(jd == epo);
        let epo = Julian::from(TS3_JULIAN_EPOCH);
        assert!(jd == epo);
        let epo = Julian::from(TS3_JULIAN_EPOCH as u32);
        assert!(jd == epo);
    }

    #[test]
    fn test_date() {
        let jd: Julian = Default::default();
        let (y, m, d) = jd.date();
        assert_eq!(y, 1970);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
        let jd = Julian::new_jdn(2018, 10, 1).unwrap();
        let jd1 = Julian::from(jd.jdn());
        assert!(jd == jd1);
        let (y, m, d) = jd.date();
        assert_eq!(y, 2018);
        assert_eq!(m, 10);
        assert_eq!(d, 1);
        println!("dump julian(2018,10,1): {}", jd);
        let jd1 = Julian::from((2018, 10, 1));
        assert!(jd == jd1);
    }
}
