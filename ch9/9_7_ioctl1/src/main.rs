extern crate libc;

use std::os::unix::io::AsRawFd;
use std::io;

// 터미널 I/O 제어 명령어
const TIOCGWINSZ: libc::c_ulong = 0x5413;

// Winsize데이터 형식
#[repr(C)]
struct Winsize {
    ws_row: libc::c_ushort,    // 문자 단위의 행 수
    ws_col: libc::c_ushort,    // 문자 단위의 열 수
    ws_xpixel: libc::c_ushort, // 가로 픽셀 크기
    ws_ypixel: libc::c_ushort, // 세로 픽셀 크기
}

fn get_terminal_size() -> Result<(libc::c_ushort, libc::c_ushort), io::Error> {
    let ws = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let fd = io::stdout().as_raw_fd();
    // ioctl을 사용하여 ws구조체 데이터를 획득
    let result = unsafe {
        libc::ioctl(fd, TIOCGWINSZ, &ws)
    };

    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok((ws.ws_col, ws.ws_row))
    }
}

fn main() {
    match get_terminal_size() {
        Ok((width, height)) => {
            println!("Terminal size: {}x{}", width, height);
        },
        Err(e) => {
            eprintln!("Failed to get terminal size: {}", e);
        }
    }
}