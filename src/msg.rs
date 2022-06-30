use std::fmt;
use std::mem;
use std::ops::{Add, AddAssign};

const CLMSG_CAP: usize = 62;
#[repr(C, align(64))]
#[derive(Eq, Copy, Clone)]
pub struct ClMessage {
    len: u16,
    da: [u8; CLMSG_CAP],
}

impl Default for ClMessage {
    fn default() -> Self {
        let da = mem::MaybeUninit::<[u8; CLMSG_CAP]>::uninit();
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
        assert!(self.len() < CLMSG_CAP);
        let len = self.len + 1;
        let mut da = self.da.clone();
        da[self.len()] = rhs;
        ClMessage { len, da }
    }
}

impl AddAssign<u8> for ClMessage {
    fn add_assign(&mut self, rhs: u8) {
        assert!(self.len() < CLMSG_CAP);
        self.da[self.len()] = rhs;
        self.len += 1;
    }
}

impl Add<&[u8]> for ClMessage {
    type Output = ClMessage;
    fn add(self, rhs: &[u8]) -> ClMessage {
        let dlen = self.len as usize;
        let len = rhs.len() + dlen;
        assert!(len <= CLMSG_CAP);
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
        assert!(len <= CLMSG_CAP);
        self.da[dlen..len].copy_from_slice(rhs);
        self.len = len as u16;
    }
}

impl ClMessage {
    pub fn new(src: &[u8]) -> ClMessage {
        let len: u16 = if src.len() > CLMSG_CAP {
            CLMSG_CAP as u16
        } else {
            src.len() as u16
        };
        let da = mem::MaybeUninit::<[u8; CLMSG_CAP]>::uninit();
        let mut da = unsafe { da.assume_init() };
        da[..len as usize].copy_from_slice(&src[..len as usize]);
        ClMessage { len, da }
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn data(&self) -> &[u8] {
        &self.da[..self.len()]
    }
    pub fn cap(&self) -> usize {
        mem::size_of_val(&self.da)
    }
    pub fn free_space(&self) -> usize {
        CLMSG_CAP - self.len()
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
        assert_eq!(msg1.cap(), 62);
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
        assert_eq!(msg1.free_space(), 58);
    }
}
