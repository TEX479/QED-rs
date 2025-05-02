/*
* TODO: check for a implementation of large integers, as the python project relies on them.
* TODO: check which integers need to be a large integer, and which don't.
*/

use std::str::FromStr;

use large_int::large_int::LargeInt;

struct Key {
    // check for implementations of large integers. i64 will not be sufficient.
    normal: Vec<i64>,
    start: i64,
    mix: i64,
    cube: LargeInt,
}

pub fn get_key_m_cube(key_normal: Vec<LargeInt>, key_start: i64, g: Option<i64>) -> LargeInt {
    todo!()
}

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub fn encrypt() {}

pub fn _mix_letter(
    way: bool,
    text: LargeInt,
    key: Vec<usize>,
    l2: LargeInt,
    chunk: usize,
) -> LargeInt {
    // TODO: add _mix_letter
    todo!()
}
