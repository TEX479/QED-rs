use std::{fmt::Debug, str::FromStr};

use large_int::large_int::LargeInt;

use crate::hilfsfunktionen;
// use std::{fmt::Debug, ops::Index};

struct Cube<T> {
    cube: [Vec<Vec<T>>; 6],
    dimensions: usize,
}

fn clone_2dvector<T: Clone>(vector: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut vector_new: Vec<Vec<T>> = Vec::new();
    for vec_inner in vector.iter() {
        vector_new.push(vec_inner.clone());
    }
    vector_new
}

fn rotate_2dvector<T: Clone + Debug>(
    vector: &Vec<Vec<T>>,
    rotations: i32,
) -> Result<Vec<Vec<T>>, String> {
    if vector.is_empty() {
        return Ok(clone_2dvector(vector));
    }

    let inside_vector_length = vector[0].len();
    for inside_vector in vector.iter() {
        if inside_vector.len() != inside_vector_length {
            return Err(
                format!("Can not rotate non-rectangle 2D vectors\n{:?}", vector).to_string(),
            );
        }
    }

    let mut vector_new = clone_2dvector(vector);

    for _ in 0..(rotations % 4) {
        // single rotation
        let mut vector_in_construction: Vec<Vec<T>> = Vec::new();
        for x in 0..inside_vector_length {
            vector_in_construction.push(Vec::new());
            for y in (0..vector.len()).rev() {
                vector_in_construction[x].push(vector_new[y][x].clone());
            }
        }
        vector_new = vector_in_construction;
    }

    Ok(vector_new)
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl From<i32> for Axis {
    fn from(value: i32) -> Self {
        /*
         * WARNING: THIS FUNCTION COULD THEORETICALLY PANIC!!!
         */
        let option = value % 3;
        if option == 0 {
            Axis::X
        } else if option == 1 {
            Axis::Y
        } else if option == 2 {
            Axis::Z
        } else {
            panic!(
                "any value modulo 3 should be either 0, 1 or 2, not {:?}",
                option
            )
        }
    }
}

// impl<T: ... + Index<usize>> Cube<T>
impl<T: Clone + Debug> Cube<T> {
    fn new(dimensions: usize) -> Self {
        /*
        let plane = {
            let mut column = Vec::new();
            for _ in 0..dimensions {
                column.push(T::new());
            }
            let mut plane = Vec::new();
            for _ in 0..dimensions {
                plane.push(column.clone());
            }
            plane
        };
        */
        let plane: Vec<Vec<T>> = Vec::new();
        Cube {
            cube: [
                plane.clone(),
                plane.clone(),
                plane.clone(),
                plane.clone(),
                plane.clone(),
                plane.clone(),
            ],
            dimensions,
        }
    }

    fn rotate(&mut self, axis: Axis, plane: usize, rotation: i32) -> Result<(), String> {
        let rotation_processed = match &axis {
            Axis::Z => (4 - rotation) % 4,
            _ => rotation % 4,
        };
        for _ in 0..rotation_processed {
            self._rotate(axis, plane)?;
        }
        Ok(())
    }

    fn _rotate(&mut self, axis: Axis, plane: usize) -> Result<(), String> {
        let (rotations, faces): ([i32; 4], [usize; 4]) = match axis {
            Axis::X => ([0, 0, 0, 2], [0, 2, 5, 4]),
            Axis::Y => ([1, 0, 3, 2], [0, 3, 5, 1]),
            Axis::Z => ([1, 1, 1, 1], [4, 3, 2, 1]),
        };

        let mut full_rotation = Vec::new();
        for i in 0..6 {
            for (face_index, face) in faces.iter().enumerate() {
                if i == *face {
                    full_rotation.push(rotations[face_index]);
                }
            }
            if full_rotation.len() != i + 1 {
                full_rotation.push(0);
            }
        }

        let mut cube_r: Vec<Vec<Vec<T>>> = Vec::new();

        #[allow(clippy::needless_range_loop)]
        // `i` is used for indexing another iterable, not just `full_rotation`
        for i in 0..6 {
            let rotated_vector = rotate_2dvector(&self.cube[i], full_rotation[i] + 1)?;
            cube_r.push(rotated_vector)
        }

        let last_face = match faces.last() {
            Some(face) => face,
            None => return Err("in \"cube._rotate\", somehow the \"facecs\" variable had no last face. this by all means should never happen, as the faces are hard coded".to_string()),
        };
        let mut h2 = cube_r[*last_face][plane].clone();
        for face in faces {
            let h1 = cube_r[face][plane].clone();
            cube_r[face] = cube_r[face].clone(); // What did I smoke when I wrote this in the original?
            cube_r[face][plane] = h2;
            h2 = h1;
        }
        for i in 0..6 {
            self.cube[i] = rotate_2dvector(&cube_r[i], 4 - full_rotation[i] - 1)?;
        }

        if !((plane == 0) || (plane + 1 == self.dimensions)) {
            return Ok(());
        }
        let cube_index = if plane == 0 {
            match axis {
                Axis::X => 1,
                Axis::Y => 2,
                Axis::Z => 0,
            }
        } else {
            match axis {
                Axis::X => 3,
                Axis::Y => 4,
                Axis::Z => 5,
            }
        };
        self.cube[cube_index] = rotate_2dvector(&self.cube[cube_index], 1)?;
        Ok(())
    }
}
impl Cube<LargeInt> {
    fn from(&mut self, mut text: LargeInt, cube_field_data_size_bits: usize) -> Result<(), String> {
        /*
         *  changes self, even if it errors, so don't proceed when an Err() is returned!
         */

        let text_length = hilfsfunktionen::get_number_length(&text);
        let maximum_writable_length =
            self.dimensions * self.dimensions * 6 * cube_field_data_size_bits;
        match maximum_writable_length.cmp(&text_length) {
            std::cmp::Ordering::Greater => {
                return Err("Can not write provided text to cube: Text is too short.".to_string());
            }
            std::cmp::Ordering::Equal => {
                text /= 2i64.pow((text_length - maximum_writable_length) as u32)
            }
            std::cmp::Ordering::Less => (),
        }

        for plane_index in 0..6 {
            let mut plane = Vec::new();
            for x in 0..self.dimensions {
                plane.push(Vec::new());
                for y in 0..self.dimensions {
                    let shift = maximum_writable_length
                        - (((plane_index * self.dimensions * self.dimensions)
                            + (x * self.dimensions)
                            + y)
                            * cube_field_data_size_bits);
                    plane[x].push(
                        (text.clone() >> shift) & (2i64.pow(cube_field_data_size_bits as u32) - 1),
                    );
                }
            }
            self.cube[plane_index] = plane;
        }

        Ok(())
    }

    fn to(&mut self, cube_field_data_size_bits: usize) -> LargeInt {
        /*
         * modifies self, so if result is Err(), self may be unusable!
         *  if cube_field_data_size_bits does not match the value of when the data was written to
         *  the cube, the result is "random"/unusable.
         */
        let mut text = LargeInt::from(0);

        for plane_index in 0..6 {
            for x in 0..self.dimensions {
                for y in 0..self.dimensions {
                    text += self.cube[plane_index][x][y].clone();
                    text *= 2i64.pow(cube_field_data_size_bits as u32);
                }
            }
        }

        text
    }
}

pub fn cube(
    mut text: LargeInt,
    key_m_cube: LargeInt,
    // cube_dimensions: usize,
    encryption: bool,
    self_l: usize,
) -> Result<LargeInt, String> {
    if (self_l >= (20 * 20 * 6)) && encryption {
        let cube_field_data_size_local: usize = self_l / (20 * 20 * 6);
        let key_m_cube_big = crate::qed_system::get_key_m_cube(
            hilfsfunktionen::int2anybase(key_m_cube.clone(), LargeInt::from(42)),
            343,
            Some(1_000),
        );
        text = cube_big(
            text,
            key_m_cube_big,
            20,
            cube_field_data_size_local,
            encryption,
        )?;
    }

    let step_array = hilfsfunktionen::int2anybase(key_m_cube.clone(), LargeInt::from(18));
    use crate::constants::{KEY_M_CUBE_2_INITIAL_STR, QUICK_ROTATE};
    let mut key_m_cube_2 = LargeInt::from_str(KEY_M_CUBE_2_INITIAL_STR).unwrap();
    use crate::qed_system::_mix_letter;
    for i2 in step_array {
        let i2_usize: usize = i2.try_into().unwrap();
        key_m_cube_2 = _mix_letter(
            false,
            key_m_cube_2,
            QUICK_ROTATE[i2_usize].to_vec(),
            LargeInt::from(216 * 8),
            8,
        );
    }
    let key_m_cube_2_bitlen = hilfsfunktionen::get_number_length(&key_m_cube_2);
    if key_m_cube_2_bitlen > (216 * 8) {
        key_m_cube_2 >>= key_m_cube_2_bitlen - (216 * 8);
    }

    let mut key_m_cube_2_bytevec: Vec<usize> = Vec::new();
    for _ in 0..216 {
        key_m_cube_2_bytevec.push((key_m_cube_2.clone() & 0xFF).try_into().unwrap());
        key_m_cube_2 >>= 8;
    }
    key_m_cube_2_bytevec.reverse();

    let mut text_scrambled = _mix_letter(encryption, text, key_m_cube_2_bytevec, self_l.into(), 1);

    if (self_l >= (20 * 20 * 6)) && (!encryption) {
        let cube_field_data_size_local = self_l / (20 * 20 * 6);
        let key_m_cube_big = crate::qed_system::get_key_m_cube(
            hilfsfunktionen::int2anybase(key_m_cube, 42.into()),
            343,
            Some(1_000),
        );
        text_scrambled = cube_big(
            text_scrambled,
            key_m_cube_big,
            20,
            cube_field_data_size_local,
            encryption,
        )?;
    }

    Ok(text_scrambled)
}

fn cube_big(
    text: LargeInt,
    key_m_cube: LargeInt,
    cube_dimensions: usize,
    cube_field_data_size: usize,
    encryption: bool,
) -> Result<LargeInt, String> {
    let lshift = cube_dimensions.pow(2) * 6 * cube_field_data_size;
    let text_formatted = text.clone() & (LargeInt::from(2).pow(lshift as u32) - 1);

    let mut cube_instance: Cube<LargeInt> = Cube::new(cube_dimensions);
    cube_instance.from(text_formatted, cube_field_data_size)?;

    let step_vec = _cube_int_to_moves(key_m_cube, cube_dimensions, encryption)?;

    let rotation = {
        if encryption {
            1
        } else {
            3 // = (-1) % 4
        }
    };
    for (axis, plane) in step_vec {
        cube_instance.rotate(axis, plane, rotation)?;
    }

    Ok(((text >> lshift) << lshift) + cube_instance.to(cube_field_data_size))
}

fn _cube_int_to_moves(
    key_m_cube: LargeInt,
    cube_dimensions: usize,
    encryption: bool,
) -> Result<Vec<(Axis, usize)>, String> {
    let mut step_vec: Vec<(Axis, usize)> = Vec::new();
    let seed = hilfsfunktionen::int2anybase(key_m_cube, LargeInt::from(cube_dimensions * 3));

    for value in seed {
        let value_i64: i64 = match value.try_into() {
            Ok(number) => number,
            Err(error) => {
                return Err(format!(
                    "Can not convert element of seed to i32: {:?}",
                    error
                ));
            }
        };
        let axis = Axis::from(value_i64 as i32);
        let plane: usize = value_i64 as usize / 3;

        step_vec.push((axis, plane));
    }

    if !encryption {
        step_vec.reverse();
    }

    Ok(step_vec)
}
