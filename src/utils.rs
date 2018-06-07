use std::time::{SystemTime, UNIX_EPOCH};

pub const CHARS: &'static [u8] = b"0123456789abcdef";

pub fn timestamp_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

pub fn to_hex(v: &Vec<u8>) -> String {
    let mut res = Vec::with_capacity(v.len() * 2);
    for b in v {
        res.push(CHARS[(b >> 4) as usize]);
        res.push(CHARS[(b & 0xf) as usize]);
    }
    String::from_utf8(res).unwrap()
}
