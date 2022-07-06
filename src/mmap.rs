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

impl Mmap {
    pub fn new(len: usize, fp: &str, hugepage: bool, read_only: bool) -> Mmap {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_exist() {
        assert!(file_readable("/dev/null"));
        println!("/dev/null test ok");
        assert!(!file_readable("/tmp/abc"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_hp_path() {
        let ss = "mdseries.bin";
        if let Some(fpath) = hp_path(ss) {
            println!("got {} shared memory path: {}", ss, fpath);
        } else {
            println!("no {} on shared memory", ss);
        }
    }
}
