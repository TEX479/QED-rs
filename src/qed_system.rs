/*
* TODO: check for a implementation of large integers, as the python project relies on them.
* TODO: check which integers need to be a large integer, and which don't.
*/

use crate::cube;
use std::str::FromStr;

struct Key {
    // check for implementations of large integers. i64 will not be sufficient.
    normal: Vec<i64>,
    start: i64,
    mix: i64,
    cube: i64,
}

fn _get_key_m_cube(key_normal: Vec<i64>, key_start: i64, g: Option<i64>) -> i64 {
    todo!()
}

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub fn encrypt() {}
