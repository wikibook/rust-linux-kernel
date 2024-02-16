extern crate env_logger;

use std::env;
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use libc::ENOENT;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

const TTL: Duration = Duration::from_secs(1);           // 1 second

// 루트 디렉토리에 대한 파일 특성을 정의합니다.
const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

// "hello.txt" 파일의 내용을 정의합니다.
const HELLO_TXT_CONTENT: &str = "Hello World!\n";

// "hello.txt" 파일에 대한 파일 특성을 정의합니다.
const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

// HelloFS 파일 시스템 구조를 정의합니다.
struct HelloFS;

// Filesystem 트레잇을 구현하여 HelloFS에 기능을 추가합니다.
impl Filesystem for HelloFS {
    // 주어진 이름과 부모 inode를 사용하여 파일 또는 디렉토리를 찾습니다.
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    // 주어진 inode를 사용하여 파일 또는 디렉토리의 특성을 얻습니다.
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    // 주어진 inode와 오프셋을 사용하여 파일의 내용을 읽습니다.
    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
        if ino == 2 {
            // HelloWorld를 출력합니다.
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    // 주어진 inode를 사용하여 디렉토리의 내용을 읽습니다.
    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "hello.txt"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
        }
        reply.ok();
    }
}

fn main() {
    env_logger::init();

    // 파라미터의 마운트 경로를 확인합니다.
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    
    // HelloFS를 지정된 마운트 지점에 마운트합니다.
    fuse::mount(HelloFS, mountpoint, &options).unwrap();
}