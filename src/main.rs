extern crate memcmp;

use std::io::prelude::*;
use std::io;

mod search;


fn main() {
    let r = io::stdin();
    search::search(r).unwrap();
}

