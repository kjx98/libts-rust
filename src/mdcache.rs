use super::mmap::{hp_path, Mmap};
use libc::c_void;
use std::fs::File;
use std::io::{Read, Result};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MdHeader {
    pub init_time: u64,
    pub shut_time: u64,
    pub max_messages: u64,
    pub cnt_messages: u64,
    pub rec_size: i32,
    pub sesson_no: i32,
    pub md_len: u64,
}

impl MdHeader {
    pub fn new() -> Result<MdHeader> {
        use std::mem::MaybeUninit;
        let fpath = hp_path("mdseries.bin")?;
        let mut fd = File::open(&fpath)?;
        let mut buf = unsafe { MaybeUninit::<[u8; 64]>::uninit().assume_init() };
        let _rlen = fd.read(&mut buf[..])?;
        let md_ptr = &buf as *const u8;
        let md_ptr: *const MdHeader = md_ptr.cast();
        let res = unsafe { *md_ptr };
        Ok(res)
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
            let mut map = Mmap::new("mdseries.bin", md.md_len as usize, true, true);
            map.open();
            assert!(!map.is_null());
            let _md = unsafe { &(*(map.mut_ptr() as *const MdHeader)) };
        } else {
            assert!(false, "MdHeader::new()");
        }
    }
}
