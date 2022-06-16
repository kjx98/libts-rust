use std::arch;

// x86_64 CPU support rdtscp
#[cfg(target_arch = "x86_64")]
pub fn check_rdtscp() -> bool {
    let result = unsafe { arch::x86_64::__cpuid_count(0x80000001u32, 0) };
    (result.edx & (1 << 27)) != 0
}

// x86_64 CPU support 1Gb hugepages
#[cfg(target_arch = "x86_64")]
pub fn check_pdpe1gb() -> bool {
    let result = unsafe { arch::x86_64::__cpuid_count(0x80000001u32, 0) };
    (result.edx & (1 << 26)) != 0
}

#[cfg(target_arch = "x86_64")]
pub fn x86_cpuid() -> u64 {
    let result = unsafe { arch::x86_64::__cpuid_count(1u32, 0) };
    let res: u64 = (result.edx as u64) << 32;
    let res = res | result.eax as u64;
    res
}

#[cfg(target_arch = "x86_64")]
pub fn rdtscp() -> u64 {
    use std::arch::asm;
    let edx: u32;
    let eax: u32;
    unsafe {
        asm!("rdtscp",
            out("edx") edx, out("eax") eax, out("cx") _);
    }
    ((edx as u64) << 32) + (eax as u64)
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
mod tests {
    use super::*;

    #[test]
    fn test_x86cpu() {
        if check_rdtscp() {
            println!("x86_64 CPU support rdtscp!");
        }
        if check_pdpe1gb() {
            println!("x86_64 CPU support 1G HugePages!");
        }
        println!("x86_64 cpu_id: {:x}", x86_cpuid());
    }
}
