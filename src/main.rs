extern crate memcmp;

use std::io::prelude::*;
use std::io;

use memcmp::Memcmp;


fn main() {
    let mut r = io::stdin();
    search(&mut r).unwrap();
}

fn read_exact<R: Read>(r: &mut R, mut buf: &mut [u8]) -> io::Result<usize> {
    let mut pos = 0;
    while !buf.is_empty() {
        let c = try!(r.read(&mut buf[pos..]));
        if c == 0 {
            break;
        }
        pos += c;
    }
    Ok(pos)
}

fn search<R: Read>(r: &mut R) -> io::Result<()> {
    let pattern = b"mdat";

    let read_len = 1024 * 1024;
    let buf_len = read_len + pattern.len();
    let mut buf = Vec::with_capacity(buf_len);
    unsafe { buf.set_len(buf_len) };

    let mut offset = 0u64;
    loop {
        let c = try!(read_exact(r, &mut buf[..read_len]));
        if c == 0 {
            break;
        }
        let buf = &buf[..c];
        
        for i in 0..(buf.len() - pattern.len()) {
            if pattern.memcmp(&buf[i..(i + pattern.len())]) {
                let match_pos = offset + i as u64;
                println!("match at {}", match_pos);
            }
        }

        offset += c as u64;
    }

    Ok(())
}
