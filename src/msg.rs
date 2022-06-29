use std::fmt;
use std::mem;
use std::ops::{Add, AddAssign};

#[repr(C, align(64))]
#[derive(Eq, Copy, Clone)]
pub struct ClMessage {
    len: u16,
    da: [u8; 62],
}

impl Default for ClMessage {
    fn default() -> Self {
        let da = mem::MaybeUninit::<[u8; 62]>::uninit();
        let da = unsafe { da.assume_init() };
        ClMessage { len: 0, da }
    }
}

impl fmt::Display for ClMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "len: {}, data: 0x{:02x}{:02x}{:02x}...",
            self.len, self.da[0], self.da[1], self.da[2]
        )
    }
}

impl PartialEq for ClMessage {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.da[..self.len as usize] == other.da[..self.len as usize]
    }
}

impl From<&str> for ClMessage {
    fn from(src: &str) -> ClMessage {
        let src = src.as_bytes();
        ClMessage::new(src)
    }
}

impl Add<u8> for ClMessage {
    type Output = ClMessage;
    fn add(self, rhs: u8) -> ClMessage {
        assert!(self.len < 62);
        let len = self.len + 1;
        let mut da = self.da.clone();
        da[self.len as usize] = rhs;
        ClMessage { len, da }
    }
}

impl AddAssign<u8> for ClMessage {
    fn add_assign(&mut self, rhs: u8) {
        assert!(self.len < 62);
        self.da[self.len as usize] = rhs;
        self.len += 1;
    }
}

impl Add<&[u8]> for ClMessage {
    type Output = ClMessage;
    fn add(self, rhs: &[u8]) -> ClMessage {
        let dlen = self.len as usize;
        let len = rhs.len() + dlen;
        assert!(len <= 62);
        let mut da = self.da.clone();
        da[dlen..len].copy_from_slice(rhs);
        let len = len as u16;
        ClMessage { len, da }
    }
}

impl AddAssign<&[u8]> for ClMessage {
    fn add_assign(&mut self, rhs: &[u8]) {
        let dlen = self.len as usize;
        let len = rhs.len() + dlen;
        assert!(len <= 62);
        self.da[dlen..len].copy_from_slice(rhs);
        self.len = len as u16;
    }
}

impl ClMessage {
    pub fn new(src: &[u8]) -> ClMessage {
        let len: u16 = if src.len() > 62 { 62 } else { src.len() as u16 };
        let da = mem::MaybeUninit::<[u8; 62]>::uninit();
        let mut da = unsafe { da.assume_init() };
        da[..len as usize].copy_from_slice(&src[..len as usize]);
        ClMessage { len, da }
    }
    pub fn len(&self) -> u16 {
        self.len
    }
    pub fn data(&self) -> &[u8] {
        &self.da[..self.len as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_sizeof_msg() {
        assert_eq!(mem::size_of::<ClMessage>(), 64);
        assert_eq!(mem::align_of::<ClMessage>(), 64);
    }

    #[test]
    fn test_convert() {
        let msg1 = ClMessage::from("test");
        let da: [u8; 4] = [b't', b'e', b's', b't'];
        assert_eq!(msg1.len(), 4);
        assert_eq!(*msg1.data(), da[..]);
        let msg2 = ClMessage::new(&da[..]);
        assert!(msg1 == msg2);
        let msg1 = ClMessage::from("tes");
        let msg1 = msg1 + b't';
        assert!(msg1 == msg2);
        let mut msg1 = ClMessage::from("te");
        msg1 += &da[2..];
        assert!(msg1 == msg2);
        let msg1 = ClMessage::from("te");
        let msg1 = msg1 + &da[2..];
        assert!(msg1 == msg2);
    }
}
