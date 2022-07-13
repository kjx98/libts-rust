use libc::{c_int, c_void, size_t};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result};
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "linux")]
fn file_readable(f: &str) -> bool {
    if let Ok(_f) = File::open(f) {
        true
    } else {
        false
    }
}

#[cfg(target_os = "linux")]
fn file_len(f: &str) -> Result<u64> {
    use std::io::{Seek, SeekFrom};
    let mut fd = File::open(f)?;
    fd.seek(SeekFrom::End(0))
}

// hugetlbfs based shared data path
#[cfg(target_os = "linux")]
pub fn hp_path(fb: &str) -> Result<String> {
    let mut f = File::open("/proc/mounts")?;
    let mut ss = String::new();
    _ = f.read_to_string(&mut ss);
    let mut it = ss.lines();
    let mut sf_len = 0u64;
    let mut res = String::new();
    {
        let fpath = "/dev/shm/".to_owned() + fb;
        if file_readable(&fpath) {
            #[cfg(test)]
            println!("may use {} as shared memory path", &fpath);
            let flen = file_len(&fpath)?;
            if flen >= sf_len {
                sf_len = flen;
            }
        }
    }
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
            println!("may use {} as shared memory path", &fpath);
            let flen = file_len(&fpath)?;
            if flen >= sf_len {
                res = fpath;
                sf_len = flen;
            }
        }
    }
    _ = file_len(&res)?;
    #[cfg(test)]
    if res.len() > 0 {
        println!("selected {} size({}) as shared memory", &res, sf_len);
    }
    Ok(res)
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
    pub fn new(fp: &str, len: u64, hugepage: bool, read_only: bool) -> Mmap {
        let len = len as size_t;
        let base = 0 as *mut c_void;
        let mut flags: c_int = if hugepage {
            libc::MAP_SHARED | libc::MAP_HUGETLB | libc::MAP_POPULATE
        } else {
            libc::MAP_SHARED
        };
        let path = if !hugepage {
            "/dev/shm/".to_owned() + fp
        } else {
            if let Ok(ss) = hp_path(fp) {
                ss
            } else {
                flags = libc::MAP_SHARED;
                "/dev/shm/".to_owned() + fp
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
            if let Ok(ll) = file_len(&self.path) {
                if ll == 0 {
                    return false;
                }
            } else {
                return false;
            }
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
        let prot = if self.read_only {
            libc::PROT_READ
        } else {
            libc::PROT_READ | libc::PROT_WRITE
        };
        unsafe {
            self.base = libc::mmap(
                nullptr,
                self.len,
                prot,
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
    pub fn ptr(&self) -> *const c_void {
        self.base as *const c_void
    }
    pub fn mut_ptr(&mut self) -> *mut c_void {
        self.base
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_null(&self) -> bool {
        self.base.is_null()
    }
    pub fn as_bytes(&self) -> Result<&[u8]> {
        if self.base.is_null() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        let slice = unsafe { &(*std::ptr::slice_from_raw_parts(self.base as *const u8, self.len)) };
        Ok(slice)
    }
    pub fn as_slice<T: Sized>(&self) -> Result<&[T]> {
        if self.base.is_null() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        let slen = std::mem::size_of::<T>();
        let slen = self.len() / slen;
        if slen == 0 {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        let addr = self.base as *const T;
        let slice = unsafe { &(*std::ptr::slice_from_raw_parts(addr, slen)) };
        Ok(slice)
    }
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
        if let Ok(fpath) = hp_path(ss) {
            println!("got {} shared memory path: {}", ss, fpath);
        } else {
            println!("no {} on shared memory", ss);
        }
    }

    #[test]
    fn test_mmap() {
        //let mut map = Mmap::new("mdseries.bin", 2147483648, true, false);
        let mut map = Mmap::new("mdseries.bin", 2147483648, true, true);
        println!("mmap open: {}", map.open());
        let addr = map.mut_ptr();
        let addr1 = unsafe { addr.add(64) };
        assert!(addr != addr1);
        println!("addr/addr1: {:x}/{:x}", addr as usize, addr1 as usize);
    }
}
