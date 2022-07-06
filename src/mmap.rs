use libc::{c_int, c_void, size_t};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "linux")]
fn file_readable(f: &str) -> bool {
    if let Ok(_f) = File::open(f) {
        true
    } else {
        false
    }
}

// hugetlbfs based shared data path
#[cfg(target_os = "linux")]
pub fn hp_path(fb: &str) -> Option<String> {
    if let Ok(mut f) = File::open("/proc/mounts") {
        let mut ss = String::new();
        _ = f.read_to_string(&mut ss);
        let mut it = ss.lines();
        while let Some(aline) = it.next() {
            let v: Vec<&str> = aline.split(' ').collect();
            if v.len() < 4 {
                continue;
            }
            if v[0] == "overlay" && v[1] == "/" {
                #[cfg(test)]
                println!(
                    "overlay mount {} type {}, running within container",
                    v[1], v[2]
                );
                continue;
            }
            if v[2] != "hugetlbfs" {
                continue;
            }
            let fpath: String = v[1].to_owned() + "/" + fb;
            if file_readable(&fpath) {
                #[cfg(test)]
                println!("use {} as shared memory path", &fpath);
                return Some(fpath);
            }
        }
    }
    None
}

pub struct Mmap {
    base: *mut c_void,
    len: size_t,
    flags: c_int,
    path: String,
    read_only: bool,
}

impl Drop for Mmap {
    fn drop(&mut self) {
        #[cfg(test)]
        println!("call mmap.close() from Drop");
        self.close();
    }
}

impl Mmap {
    pub fn new(fp: &str, len: usize, hugepage: bool, read_only: bool) -> Mmap {
        let len = len as size_t;
        let base = 0 as *mut c_void;
        let flags: c_int = if hugepage {
            libc::MAP_HUGETLB | libc::MAP_POPULATE
        } else {
            0
        };
        let path = if !hugepage {
            "/dev/mem/".to_owned() + fp
        } else {
            if let Some(ss) = hp_path(fp) {
                ss
            } else {
                "/dev/mem/".to_owned() + fp
            }
        };
        Mmap {
            base,
            len,
            flags,
            path,
            read_only,
        }
    }
    pub fn open(&mut self) -> bool {
        let nullptr = 0 as *mut c_void;
        let fd = if self.read_only {
            if let Ok(fd) = OpenOptions::new().read(true).open(&self.path) {
                fd
            } else {
                return false;
            }
        } else {
            if let Ok(fd) = OpenOptions::new().read(true).write(true).open(&self.path) {
                _ = fd.set_len(self.len as u64);
                fd
            } else {
                return false;
            }
        };
        if self.read_only {
            unsafe {
                self.base = libc::mmap(
                    nullptr,
                    self.len,
                    libc::PROT_READ,
                    libc::MAP_SHARED | self.flags,
                    fd.as_raw_fd(),
                    0,
                );
            }
        } else {
            unsafe {
                self.base = libc::mmap(
                    nullptr,
                    self.len,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED | self.flags,
                    fd.as_raw_fd(),
                    0,
                );
                #[cfg(test)]
                if self.base.is_null() {
                    use std::ffi::CString;
                    let s = CString::new("mmap failed").expect("CString failed");
                    libc::perror(s.as_ptr());
                }
            }
        }
        self.base != nullptr
    }
    pub fn close(&mut self) {
        if self.base.is_null() {
            return;
        }
        unsafe {
            if libc::munmap(self.base, self.len) == 0 {
                self.base = 0 as *mut c_void;
            }
        }
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.base as *const u8
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_null(&self) -> bool {
        self.base.is_null()
    }
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if self.base.is_null() {
            return None;
        }
        let slice = unsafe { &(*std::ptr::slice_from_raw_parts(self.as_ptr(), self.len())) };
        Some(slice)
    }
}

#[repr(C)]
pub struct MdHeader {
    init_time: u64,
    shut_time: u64,
    max_messages: u64,
    cnt_messages: u64,
    rec_size: i32,
    sesson_no: i32,
    md_len: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        assert!(file_readable("/dev/null"));
        println!("/dev/null test ok");
        assert!(!file_readable("/tmp/abc"));
    }

    #[test]
    fn test_hp_path() {
        let ss = "mdseries.bin";
        if let Some(fpath) = hp_path(ss) {
            println!("got {} shared memory path: {}", ss, fpath);
        } else {
            println!("no {} on shared memory", ss);
        }
    }

    #[test]
    fn test_mmap() {
        let mut map = Mmap::new("mdseries.bin", 2147483648, true, false);
        println!("mmap open: {}", map.open());
        if !map.is_null() {
            let md = unsafe { &(*(map.base as *const MdHeader)) };
            println!(
                "MdHeader: rec_size({}) cnt({}) len({})",
                md.rec_size, md.cnt_messages, md.md_len
            );
        }
    }
}
