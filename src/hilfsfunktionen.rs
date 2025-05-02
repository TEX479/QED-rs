use std::str::FromStr;

use large_int::large_int::LargeInt;

pub fn int2anybase(mut number: LargeInt, base: LargeInt) -> Vec<LargeInt> {
    let mut output = Vec::new();

    if number != 0 {
        while number > 0 {
            let (div_output, mod_output) = number.div_with_remainder(base.clone());
            output.push(mod_output);
            number = div_output;
        }
        output.reverse();
    }
    if output.is_empty() {
        output.push(LargeInt::from(0));
    }

    output
}

fn _get_number_length(n: LargeInt, base: u32) -> usize {
    let mut power = LargeInt::from(base);
    let mut count = 1;

    while n >= power {
        count += 1;
        power *= base;
    }

    count
}

pub fn get_number_length(n: LargeInt) -> usize {
    let n_str = format!("{:?}", n);
    let n_str_cut = n_str
        .split_once("bytes: [")
        .unwrap()
        .1
        .split_once("]")
        .unwrap()
        .0
        .replace(" ", "");
    let mut n_str_segments = Vec::new();
    for segment in n_str_cut.split(",") {
        n_str_segments.push(segment);
    }
    let u128_count = n_str_segments.len() - 1;
    let additional_bits: usize = _get_number_length(
        LargeInt::from_str(n_str_segments.last().unwrap()).unwrap(),
        2,
    );

    u128_count * 128 + additional_bits
}

// TODO: fix all of this nonsense
/*
fn anybase2anybase(input: Vec<u64>, input_base: i64, output_base: i64) -> Result<Vec<u64>, String> {
    let mut number = LargeInt::from(0);
    let input_len = input.len();
    for i in 0..input.len() {
        let shifted_input = match input.get(input_len - i - 1) {
            Some(value) => value,
            None => return Err("could not get shifted input in anybase2anybase".to_string()),
        };
        number += *shifted_input * (input_base.pow(i as u32) as u64);
    }

    let mut output_number = Vec::new();
    if number != 0 {
        while number > 0 {
            let (div_result, mod_result) = number.div_with_remainder(output_base);
            output_number.push(match mod_result.try_into() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("conversion error in anybase2anybase: {:?}", error));
                }
            });
            number = div_result;
        }
    } else {
        output_number.push(0);
    }

    Ok(output_number)
}
*/
