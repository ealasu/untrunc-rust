extern crate memcmp;

use std::io::prelude::*;
use std::io;
use std::sync::mpsc::sync_channel;
use std::thread;

use memcmp::Memcmp;


fn main() {
    let r = io::stdin();
    search(r).unwrap();
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

fn search<R: Read+Send+'static>(mut r: R) -> io::Result<()> {
    let pattern = b"mdat";

    let queue_len = 3;
    let read_len = 2 * 1024 * 1024;
    let buf_len = read_len + pattern.len();

    let (empty_tx, empty_rx) = sync_channel(queue_len);
    let (full_tx, full_rx) = sync_channel(queue_len);
    for _ in 0..queue_len {
        let mut buf = Vec::with_capacity(buf_len);
        unsafe { buf.set_len(buf_len) };
        empty_tx.send(buf).unwrap();
    }

    thread::spawn(move || {
        for mut buf in empty_rx {
            let c = read_exact(&mut r, &mut buf[..read_len]).unwrap();
            if c == 0 {
                break;
            }
            unsafe { buf.set_len(c) };
            full_tx.send(buf).unwrap();
        }
    });

    let mut offset = 0u64;
    for buf in full_rx {
        for i in 0..(buf.len() - pattern.len()) {
            if pattern.memcmp(&buf[i..(i + pattern.len())]) {
                let match_pos = offset + i as u64;
                println!("match at {}", match_pos);
            }
        }
        offset += buf.len() as u64;
        let _ = empty_tx.send(buf);
    }

    Ok(())
}
