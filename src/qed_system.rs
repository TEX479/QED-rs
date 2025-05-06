/*
* TODO: check for a implementation of large integers, as the python project relies on them.
* TODO: check which integers need to be a large integer, and which don't.
*/

use std::str::FromStr;

use large_int::large_int::LargeInt;

struct Key {
    // check for implementations of large integers. i64 will not be sufficient.
    normal: Vec<LargeInt>,
    start: LargeInt,
    mix: LargeInt,
    cube: LargeInt,
}

impl Key {
    fn from_u8s(key: Vec<u8>) -> Self {
        let mut capital_s = key;
        let mut capital_s2: Vec<Vec<u8>> = Vec::new();
        let mut capital_s3: Vec<f64> = Vec::new();
        let mut zahl2 = 0;
        let mut zahl = 0;

        let mut key_start: LargeInt = LargeInt::new(); // TODO: check if LargeInt is needed.
        let mut key_m_cube: LargeInt = LargeInt::new();
        let mut key_normal: Vec<LargeInt> = Vec::new(); // TODO: check if LargeInt is needed.
        let mut key_mix: LargeInt = LargeInt::new(); // TODO: check if LargeInt is needed.
        let mut key_len: usize = 5;

        for i in 0..(capital_s.len() / key_len) {
            capital_s2.push(
                capital_s
                    .iter()
                    .take((i + 1) * key_len)
                    .skip(i * key_len)
                    .copied()
                    .collect(),
            );
        }

        Key {
            normal: key_normal,
            start: key_start,
            mix: key_mix,
            cube: key_m_cube,
        }
    }
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
