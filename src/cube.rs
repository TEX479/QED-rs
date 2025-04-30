use std::fmt::Debug;
// use std::{fmt::Debug, ops::Index};

pub struct Cube<T> {
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
pub enum Axis {
    X,
    Y,
    Z,
}

// impl<T: ... + Index<usize>> Cube<T>
impl<T: Clone + Debug> Cube<T> {
    pub fn new(dimensions: usize) -> Self {
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

    pub fn rotate(&mut self, axis: Axis, plane: usize, rotation: i32) -> Result<(), String> {
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
