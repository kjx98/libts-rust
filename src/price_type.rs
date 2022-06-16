//use std::fmt;

#[allow(non_upper_case_globals)]
const dMulti: [f64;9] = [0.01,0.1,1.0,10.0,100.0,1000.0,10000.0,
                    100000.0,1000000.0];
#[allow(non_upper_case_globals)]
const dDiv: [f64;9] = [100.0,10.0,1.0,0.1,0.01,0.001,0.0001,
                    0.00001,0.000001];
const DIGIT_MAX: i8 = 6;
const DIGIT_MIN: i8 = -2;

fn digit_div(ndig: i8) -> f64 {
    if ndig < DIGIT_MIN || ndig > DIGIT_MAX { return 1.0; }
    dDiv[(ndig+2) as usize]
}

fn digit_multi(ndig: i8) -> f64 {
    if ndig < DIGIT_MIN || ndig > DIGIT_MAX { return 1.0; }
    dMulti[(ndig+2) as usize]
}

#[derive(Eq, Clone, Default)]
pub struct PriceType (i8);

impl PartialEq for PriceType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PriceType {
    pub const fn new(ndig: i8) -> PriceType {
        if ndig < DIGIT_MIN { PriceType(DIGIT_MIN) } else
        if ndig > DIGIT_MAX { PriceType(DIGIT_MAX) } else {
            PriceType(ndig)
        }
    }
    pub fn to_double(&self, v: i32) -> f64 {
        v as f64 * digit_div(self.0)
    }
    pub fn from_double(&self, v: f64) -> i32 {
        (v * digit_multi(self.0)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let pt: PriceType = Default::default();
        assert_eq!(pt.to_double(123), 123.0);
        assert_eq!(pt.from_double(123.0), 123);
        let pt = PriceType::new(2);
        assert_eq!(pt.to_double(123), 1.23);
        assert_eq!(pt.from_double(1.23), 123);
    }
}
