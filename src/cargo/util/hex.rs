use super::StableHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;

pub fn to_hex(num: u64) -> String {
    hex::encode(&[
        (num >> 0) as u8,
        (num >> 8) as u8,
        (num >> 16) as u8,
        (num >> 24) as u8,
        (num >> 32) as u8,
        (num >> 40) as u8,
        (num >> 48) as u8,
        (num >> 56) as u8,
    ])
}

pub fn hash_u64<H: Hash>(hashable: H) -> u64 {
    let mut hasher = StableHasher::new();
    hashable.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_u64_file(mut file: &File) -> std::io::Result<u64> {
    let mut hasher = StableHasher::new();
    let mut buf = [0; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.write(&buf[..n]);
    }
    Ok(hasher.finish())
}

pub fn short_hash<H: Hash>(hashable: &H) -> String {
    to_hex(hash_u64(hashable))
}
