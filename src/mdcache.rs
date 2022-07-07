use super::datetime::DateTimeSec;
use super::mmap::hp_path;
use super::{ClMessage, DateTime, Mmap};
use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};

const MDSERIES_PATH: &'static str = "mdseries.bin";

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MdHeader {
    pub init_time: i64,
    pub shut_time: i64,
    pub max_messages: u64,
    pub cnt_messages: u64,
    pub rec_size: i32,
    pub session_no: i32,
    pub md_len: u64,
}

impl fmt::Display for MdHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let init_t: DateTimeSec = DateTime::new(self.init_time);
        let shut_t: DateTimeSec = DateTime::new(self.shut_time);
        write!(
            f,
            "init_time: {}, shut_time: {}\nsession: {} max_msg: {}, cnt_msgs: {}",
            init_t, shut_t, self.session_no, self.max_messages, self.cnt_messages
        )
    }
}

impl MdHeader {
    pub fn new() -> Result<MdHeader> {
        use std::mem::MaybeUninit;
        let fpath = hp_path(MDSERIES_PATH)?;
        let mut fd = File::open(&fpath)?;
        let mut buf = unsafe { MaybeUninit::<[u8; 64]>::uninit().assume_init() };
        let _rlen = fd.read(&mut buf[..])?;
        let md_ptr = &buf as *const u8;
        let md_ptr: *const MdHeader = md_ptr.cast();
        let res = unsafe { *md_ptr };
        Ok(res)
    }
}

pub struct MdCache<'a> {
    #[allow(dead_code)]
    mmap: Mmap,
    md_header: &'a MdHeader,
    msgs: &'a [ClMessage],
}

impl<'a> MdCache<'a> {
    pub fn new() -> Result<MdCache<'a>> {
        let md = MdHeader::new()?;
        let mut mmap = Mmap::new(MDSERIES_PATH, md.md_len, true, false);
        if !mmap.open() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        let md_p = mmap.ptr() as *const MdHeader;
        let md_header = unsafe { &(*md_p) };
        let msg_p = unsafe { mmap.ptr().add(64) as *const ClMessage };
        let nmsg = md.max_messages as usize;
        let msgs = unsafe { &(*std::ptr::slice_from_raw_parts(msg_p, nmsg)) };
        Ok(MdCache {
            mmap,
            md_header,
            msgs,
        })
    }
    pub fn header(&self) -> &MdHeader {
        self.md_header
    }
    pub fn msgs(&self) -> &[ClMessage] {
        self.msgs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hp_path() {
        let ss = "mdseries.bin";
        if let Ok(fpath) = hp_path(ss) {
            println!("got {} shared memory path: {}", ss, fpath);
        } else {
            println!("no {} on shared memory", ss);
        }
    }

    #[test]
    fn test_mmap() {
        if let Ok(md) = MdHeader::new() {
            println!(
                "MdHeader: rec_size({}) cnt({}) len({})",
                md.rec_size, md.cnt_messages, md.md_len
            );
            let mut map = Mmap::new("mdseries.bin", md.md_len, true, true);
            map.open();
            assert!(!map.is_null());
            let _md = unsafe { &(*(map.mut_ptr() as *const MdHeader)) };
        } else {
            assert!(false, "MdHeader::new()");
        }
    }

    #[test]
    fn test_mdcache() {
        if let Ok(md) = MdCache::new() {
            println!("MdHeader: {}", md.md_header);
            assert_eq!(md.msgs.len(), md.md_header.max_messages as usize);
        }
    }
}
