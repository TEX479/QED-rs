use large_int::large_int::LargeInt;

pub mod constants;
pub mod cube;
pub mod hilfsfunktionen;
pub mod qed_system;

fn main() {
    println!("Hello, world!");
    // let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // println!("{:?}", cube::rotate_2dvector(&v, 2));
    let key_m_cube_2_1 = LargeInt::from(2).pow(216 * 8) - 2;
    /*
     let key_m_cube_str = format!(
        "{:0>padding$}",
        format!("{:b}", key_m_cube_2_1),
        padding = 216 * 8
    );
    */
    // println!("{}", key_m_cube_2_1);
    println!("{}", hilfsfunktionen::get_number_length(key_m_cube_2_1) / 8);
    /*
        let mut key_m_cube_vec = Vec::new();
        for i in 0..216 {
            key_m_cube_vec
                .push(u8::from_str_radix(&key_m_cube_str[(i * 8)..((i + 1) * 8)], 2).unwrap());
        }
        println!("{:?}", key_m_cube_vec);
    */
}
