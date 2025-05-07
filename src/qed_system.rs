use std::str::FromStr;

use large_int::large_int::LargeInt;

use crate::hilfsfunktionen::{anybase2anybase, int2anybase, int2anybase2};

struct Key {
    // check for implementations of large integers. i64 will not be sufficient.
    normal: Vec<LargeInt>,
    start: i64,
    mix: f64,
    cube: LargeInt,
}

impl Key {
    fn from_u8s(key: Vec<bool>, self_l: usize, self_chunk: usize) -> Result<Self, String> {
        // TODO: refactor "key" variable to be Vec<u8> or something similar, instead of Vec<bool>
        let capital_s = key;
        let mut capital_s2: Vec<Vec<bool>> = Vec::new();
        let mut capital_s3: Vec<f64> = Vec::new();
        let mut zahl2: Vec<bool>;
        let mut zahl = 0;
        let mut zahl2_converted;

        let mut key_start: i64;
        let key_m_cube: LargeInt =
            get_key_m_cube(key_normal.clone(), LargeInt::from(key_start), None)?;
        let mut key_normal: Vec<LargeInt> = Vec::new();
        let mut key_mix: f64 = 0.0;
        let key_len: usize = 5;

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
        if (capital_s.len() % key_len) != 0 {
            capital_s2.push(
                capital_s
                    .iter()
                    .skip((capital_s.len() / key_len) * key_len - 1)
                    .copied()
                    .collect(),
            );
        }
        if capital_s2.is_empty() {
            return Err("If `capital_s2` is empty at this point, something went wrong or no key was provided.".to_string());
        }
        let s2_last_index = capital_s2.len() - 1;
        if capital_s2[s2_last_index].len() == 1 {
            capital_s2[s2_last_index].push(false);
        }

        while zahl < capital_s2.len() {
            if !capital_s2[zahl][0] {
                if capital_s2[zahl].iter().skip(1).any(|x| *x) {
                    zahl2 = capital_s2[zahl][1..].to_vec();
                    zahl += 1;
                    while (zahl < capital_s2.len()) && (!capital_s2[zahl][0]) {
                        zahl2.extend_from_slice(&capital_s2[zahl][1..]);
                        zahl += 1;
                    }
                    zahl2_converted = {
                        let mut zahl2_tmp: i64 = 0;
                        for i in zahl2 {
                            if i {
                                zahl2_tmp += 1;
                            }
                            zahl2_tmp <<= 1;
                        }
                        zahl2_tmp as f64
                    };
                } else {
                    zahl2 = vec![false];
                    zahl += 1;
                    while (zahl < capital_s2.len()) && (!capital_s2[zahl][0]) {
                        zahl2.extend_from_slice(&capital_s2[zahl][1..]);
                        zahl += 1;
                    }
                    zahl2_converted = match f64::from_str(&format!("0.{}", {
                        let mut zahl2_tmp: i64 = 0;
                        for i in zahl2 {
                            if i {
                                zahl2_tmp += 1;
                            }
                            zahl2_tmp <<= 1;
                        }
                        zahl2_tmp
                    })) {
                        Ok(value) => value,
                        Err(msg) => {
                            return Err(format!(
                                "Parsing error during generation of `zahl2_converted`: {:?}",
                                msg
                            ));
                        }
                    };
                }
                capital_s3.push(zahl2_converted);
                key_mix += zahl2_converted;
            }

            if capital_s2[zahl][0] {
                if capital_s2[zahl].iter().skip(1).any(|x| *x) {
                    zahl2 = capital_s2[zahl][1..].to_vec();
                    zahl += 1;
                    while (zahl < capital_s2.len()) && (capital_s2[zahl][0]) {
                        zahl2.extend_from_slice(&capital_s2[zahl][1..]);
                        zahl += 1;
                    }
                    zahl2_converted = {
                        let mut zahl2_tmp: i64 = 0;
                        for i in zahl2 {
                            if i {
                                zahl2_tmp += 1;
                            }
                            zahl2_tmp <<= 1;
                        }
                        zahl2_tmp as f64
                    };
                } else {
                    zahl2 = vec![false];
                    zahl += 1;
                    while (zahl < capital_s2.len()) && (capital_s2[zahl][0]) {
                        zahl2.extend_from_slice(&capital_s2[zahl][1..]);
                        zahl += 1;
                    }
                    zahl2_converted = match f64::from_str(&format!("0.{}", {
                        let mut zahl2_tmp: i64 = 0;
                        for i in zahl2 {
                            if i {
                                zahl2_tmp += 1;
                            }
                            zahl2_tmp <<= 1;
                        }
                        zahl2_tmp
                    })) {
                        Ok(value) => value,
                        Err(msg) => {
                            return Err(format!(
                                "Parsing error during generation of `zahl2_converted`: {:?}",
                                msg
                            ));
                        }
                    };
                }
                capital_s3.push(zahl2_converted);
                key_mix += zahl2_converted;
            }
        }

        if capital_s3.len() == 1 {
            capital_s3 = vec![
                {
                    let mut zahl2_tmp: i64 = 0;
                    for &i in capital_s.iter().take(capital_s.len() / 2) {
                        if i {
                            zahl2_tmp += 1;
                        }
                        zahl2_tmp <<= 1;
                    }
                    zahl2_tmp as f64
                },
                {
                    let mut zahl2_tmp: i64 = 0;
                    for &i in capital_s.iter().skip(capital_s.len() / 2 - 1) {
                        if i {
                            zahl2_tmp += 1;
                        }
                        zahl2_tmp <<= 1;
                    }
                    zahl2_tmp as f64
                },
            ];
        }

        /*
        * the following code (outside the comment) should be a type annotated version of the
        * commented code below
                key_normal.extend(
                    capital_s3
                        .iter()
                        .skip(1)
                        .map(|&x| x.round() as i64)
                        .collect(),
                );
        */
        /*
        <std::vec::Vec<i64> as std::iter::Extend<i64>>::extend::<Vec<i64>>(
            &mut key_normal,
            capital_s3
                .iter()
                .skip(1)
                .map(|&x| x.round() as i64)
                .collect(),
        );
        */
        key_normal.extend(
            capital_s3
                .iter()
                .skip(1)
                .map(|&x| LargeInt::from(x.round() as i64)),
        );

        if capital_s3.len() == 2 {
            key_normal.push(LargeInt::from(capital_s3[1].floor() as i64 + 1));
        }

        let l_chunk_ratio = self_l as f64 / self_chunk as f64;

        if key_mix >= 1.0 {
            key_mix %= l_chunk_ratio.ceil();
            key_mix = key_mix.floor();
        } else {
            key_mix *= l_chunk_ratio.ceil();
            key_mix = key_mix.floor();
        }

        key_start = {
            if capital_s3[0] >= 1.0 {
                (capital_s3[0] % l_chunk_ratio.ceil()).floor() as i64
            } else {
                (capital_s3[0] * l_chunk_ratio.ceil()).floor() as i64
            }
        };
        if key_start == 0 {
            key_start = l_chunk_ratio.ceil() as i64 - 1;
        }

        let key_m_cube: LargeInt =
            get_key_m_cube(key_normal.clone(), LargeInt::from(key_start), None)?;

        Ok(Key {
            normal: key_normal,
            start: key_start,
            mix: key_mix,
            cube: key_m_cube,
        })
    }
}

pub fn get_key_m_cube(
    mut key_normal: Vec<LargeInt>,
    mut key_start: LargeInt,
    g: Option<i64>,
) -> Result<LargeInt, String> {
    let g = match g {
        Some(value) => value,
        None => 250,
    };

    if key_normal.iter().sum::<LargeInt>() - 10 > key_start {
        while key_start > 0 {
            if key_start > key_normal[0] {
                key_start -= key_normal.remove(0);
            } else {
                key_normal[0] -= key_start;
                key_start = LargeInt::new();
            }
        }
    }

    if key_normal.iter().sum::<LargeInt>() <= 1 {
        return Err("`key_normal.iter().sum()` <= 1".to_string());
    }

    let mut key_m_cube = key_normal.clone();
    while (key_m_cube.len() as i64) < g {
        let key_m_cube_int = match LargeInt::from_str(&match key_m_cube
            .iter()
            .map(|x| x.to_string())
            .reduce(|a, b| a + &b)
        {
            Some(value) => value,
            None => {
                return Err("Failed to parse key_m_cube_int: None is not parsable.".to_string());
            }
        }) {
            Ok(value) => value,
            Err(msg) => return Err(format!("Failed to parse key_m_cube_int: {:?}", msg)),
        };
        let key_m_cube_floats = int2anybase2(key_m_cube_int, 1.7)?;
        let key_m_cube_string = match key_m_cube_floats
            .iter()
            .map(|&x| ((x * 10.0).floor() as i64).to_string())
            .reduce(|a, b| a + &b)
        {
            Some(value) => value,
            None => return Err("`key_m_cube_string` was somehow None".to_string()),
        };
        key_m_cube = Vec::new();
        for char in key_m_cube_string.chars() {
            key_m_cube.push(match LargeInt::from_str(&char.to_string()) {
                Ok(n) => n,
                Err(msg) => {
                    return Err(format!(
                        "Could not convert Char ({:?}) to LargeInt: {:?}",
                        char, msg
                    ));
                }
            });
        }
    }
    while key_m_cube.len() as u64 > ((g as f64) * 1.75) as u64 {
        key_m_cube = anybase2anybase(key_m_cube, 10, 5);
        let key_m_cube_bak = key_m_cube.clone();
        key_m_cube = Vec::new();
        for i in 0..(key_m_cube_bak.len() / 2) {
            key_m_cube.push(key_m_cube_bak[i * 2].clone() + key_m_cube_bak[i * 2 + 1].clone());
        }
        key_m_cube = anybase2anybase(key_m_cube, 9, 10);
    }

    let key_m_cube_int = match LargeInt::from_str(&match key_m_cube
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
    {
        Some(s) => s,
        None => return Err("Could not convert `key_m_cube` to string.".to_string()),
    }) {
        Ok(n) => n,
        Err(msg) => {
            return Err(format!(
                "Could not convert string to key_m_cube_int: {:?}",
                msg
            ));
        }
    };

    key_m_cube = int2anybase(key_m_cube_int, LargeInt::from(5));
    let key_m_cube_bak = key_m_cube.clone();
    key_m_cube = Vec::new();
    for i in 0..(key_m_cube_bak.len() / 2) {
        key_m_cube.push(key_m_cube_bak[i * 2].clone() + key_m_cube_bak[i * 2 + 1].clone());
    }
    key_m_cube = anybase2anybase(key_m_cube, 9, 10);

    match LargeInt::from_str(&match key_m_cube
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
    {
        Some(s) => s,
        None => return Err("Could not convert `key_m_cube` to string.".to_string()),
    }) {
        Ok(n) => Ok(n),
        Err(msg) => {
            return Err(format!(
                "Could not convert string to key_m_cube_int: {:?}",
                msg
            ));
        }
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
