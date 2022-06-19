use std::arch::asm;
//use log::info;

#[inline]
pub fn u64_addc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let res: u64;
    let cc: u64;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "add {x}, {c}",
            "mov {c}, 0",
            "adc {c}, 0",
            "add {x}, {b}",
            "adc {c}, 0",
            x = inout(reg) a => res,
            b = in(reg) b,
            c = inout(reg) carry => cc,
            options(nomem, nostack),
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "adds {x}, {x}, {c}",
            "adc {c}, xzr, xzr",
            "adds {x}, {x}, {b}",
            "adc {c}, {c}, xzr",
            x = inout(reg) a => res,
            b = in(reg) b,
            c = inout(reg) carry => cc,
            options(nomem, nostack),
        );
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let mut cc1: bool = carry != 0;
        (res, cc1) = a.carrying_add(b, cc1);
        cc = cc1 as u64;
    }
    (res, cc)
}

#[inline]
pub fn u64_add(a: u64, b: u64) -> (u64, u64) {
    let res: u64;
    let cc: u64;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "add {x}, {b}",
            "mov {c}, 0",
            "adc {c}, 0",
            x = inlateout(reg) a => res,
            b = in(reg) b,
            c = lateout(reg) cc,
            options(nomem, nostack),
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "adds {x}, {x}, {b}",
            "adc {c}, xzr, xzr",
            x = inlateout(reg) a => res,
            b = in(reg) b,
            c = lateout(reg) cc,
            options(nomem, nostack),
        );
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let cc1: bool;
        (res, cc1) = a.carrying_add(b, false);
        cc = cc1 as u64;
    }
    (res, cc)
}

#[inline]
pub fn u64_subc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let res: u64;
    let mut cc: u64;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "sub {x}, {c}",
            "mov {c}, 0",
            "sbb {c}, 0",
            "sub {x}, {b}",
            "sbb {c}, 0",
            x = inout(reg) a => res,
            b = in(reg) b,
            c = inout(reg) carry => cc,
            options(nomem, nostack),
        );
        cc &= 1;
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "subs {x}, {x}, {c}",
            "sbc {c}, xzr, xzr",
            "subs {x}, {x}, {b}",
            "sbc {c}, {c}, xzr",
            x = inout(reg) a => res,
            b = in(reg) b,
            c = inout(reg) carry => cc,
            options(nomem, nostack),
        );
        cc &= 1;
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let mut cc1: bool = carry != 0;
        (res, cc1) = a.borrowing_sub(b, cc1);
        cc = cc1 as u64;
    }
    (res, cc)
}

#[inline]
pub fn u64_sub(a: u64, b: u64) -> (u64, u64) {
    let res: u64;
    let mut cc: u64;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "sub {x}, {b}",
            "mov {c}, 0",
            "sbb {c}, 0",
            x = inlateout(reg) a => res,
            b = in(reg) b,
            c = lateout(reg) cc,
            options(nomem, nostack),
        );
        cc &= 1;
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "subs {x}, {x}, {b}",
            "sbc {c}, xzr, xzr",
            x = inlateout(reg) a => res,
            b = in(reg) b,
            c = lateout(reg) cc,
            options(nomem, nostack),
        );
        cc &= 1;
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let cc1: bool;
        (res, cc1) = a.borrowing_sub(b, false);
        cc = cc1 as u64;
    }
    (res, cc)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_u64_subc() {
        use super::u64_subc;
        assert_eq!(u64_subc(1, 0, 0), (1, 0));
        assert_eq!(u64_subc(1, 0, 1), (0, 0));
        assert_eq!(u64_subc(1, 1, 0), (0, 0));
        assert_eq!(u64_subc(1, 1, 1), (0xffffffffffffffff, 1));
        assert_eq!(u64_subc(1, 2, 0), (0xffffffffffffffff, 1));
    }

    #[test]
    fn test_u64_sub() {
        use super::u64_sub;
        assert_eq!(u64_sub(1, 0), (1, 0));
        assert_eq!(u64_sub(1, 1), (0, 0));
        assert_eq!(u64_sub(1, 2), (0xffffffffffffffff, 1));
    }
}
