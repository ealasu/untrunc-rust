extern crate memcmp;
extern crate byteorder;

use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::fs::File;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};


fn main() {
    let mut r = File::open("/dev/mmcblk1p1").unwrap();
    parse_fat32(&mut r);
}

fn parse_fat32<R: Read+Seek>(r: &mut R) {
    r.seek(SeekFrom::Start(0x0B)).unwrap();
    let bytes_per_sector = r.read_u16::<LittleEndian>().unwrap();
    assert_eq!(bytes_per_sector, 512);

    r.seek(SeekFrom::Start(0x0D)).unwrap();
    let sectors_per_cluster = r.read_u8().unwrap();
    println!("sectors_per_cluster: {}", sectors_per_cluster);

    r.seek(SeekFrom::Start(0x0E)).unwrap();
    let reserved_sectors = r.read_u16::<LittleEndian>().unwrap();
    println!("reserved_sectors: {}", reserved_sectors);

    r.seek(SeekFrom::Start(0x10)).unwrap();
    let num_of_fats = r.read_u8().unwrap();
    assert_eq!(num_of_fats, 2);

    r.seek(SeekFrom::Start(0x24)).unwrap();
    let sectors_per_fat = r.read_u32::<LittleEndian>().unwrap();
    println!("sectors_per_fat: {}", sectors_per_fat);

    r.seek(SeekFrom::Start(0x2C)).unwrap();
    let root_dir = r.read_u32::<LittleEndian>().unwrap();
    println!("root_dir: {}", root_dir);

    r.seek(SeekFrom::Start(0x1FE)).unwrap();
    let sig = r.read_u16::<LittleEndian>().unwrap();
    assert_eq!(sig, 0xAA55);
}

