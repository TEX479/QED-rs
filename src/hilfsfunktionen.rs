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

pub fn int2anybase2(mut number: LargeInt, base: f64) -> Result<Vec<f64>, String> {
    let mut number_: Vec<f64> = Vec::new();
    if number != 0 {
        while number > 0 {
            let tmp: i128 = match ((number.clone() * 10) % (base * 10.0).floor() as i64).try_into()
            {
                Ok(number) => number,
                Err(msg) => {
                    return Err(format!(
                        "Could not convert LargeInt `number` to i128: {:?}",
                        msg
                    ));
                }
            };
            number_.push(tmp as f64 / 10.0);
            number = (number * 10) / ((base * 10.0).floor() as i64);
        }
        number_.reverse();
    } else {
        number_.push(0.0);
    }
    Ok(number_)
}

pub fn anybase2anybase(number: Vec<LargeInt>, base_input: i64, base_output: i64) -> Vec<LargeInt> {
    let mut number_ = LargeInt::new();
    for i in 0..number.len() {
        number_ += number[number.len() - i - 1].clone() * base_input.pow(i as u32);
    }

    let mut output_number: Vec<LargeInt> = Vec::new();
    if number_ != 0 {
        while number_ > 0 {
            output_number.push(number_.clone() % base_output);
            number_ /= base_output;
        }
        output_number.reverse();
    } else {
        output_number.push(LargeInt::new());
    }
    output_number
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

pub fn get_number_length(n: &LargeInt) -> usize {
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
