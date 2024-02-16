use std::mem;
use std::time::{Duration, UNIX_EPOCH};

fn main() {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        libc::gettimeofday(&mut tv, mem::zeroed());
    }

    let duration = Duration::new(tv.tv_sec as u64, tv.tv_usec as u32 * 1000);
    let system_time = UNIX_EPOCH + duration;

    let mut tm = unsafe { mem::zeroed() };
    unsafe {
        libc::localtime_r(&system_time as *const _ as *const libc::time_t, &mut tm);
    }

    let day = tm.tm_mday;
    let month = tm.tm_mon + 1; // 0부터 시작
    let year = tm.tm_year + 1900; // 1900부터 시작
    let hour = tm.tm_hour;
    let min = tm.tm_min;
    let sec = tm.tm_sec;

    println!("지금은: {}년 {}월 {}일 {}:{}:{}", year, month, day, hour, min, sec);
}