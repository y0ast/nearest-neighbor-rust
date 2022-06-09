use std::fs;

pub trait FromFourLeBytes {
    fn from_le_bytes(bytes: [u8; 4]) -> Self;
}

impl FromFourLeBytes for f32 {
    fn from_le_bytes(bytes: [u8; 4]) -> f32 {
        f32::from_le_bytes(bytes)
    }
}

impl FromFourLeBytes for i32 {
    fn from_le_bytes(bytes: [u8; 4]) -> i32 {
        i32::from_le_bytes(bytes)
    }
}

pub fn read_vecs<T: FromFourLeBytes>(file_path: &str) -> Vec<Vec<T>> {
    let data = fs::read(file_path).expect("Unable to read file");
    let d = i32::from_le_bytes((&data[..4]).try_into().unwrap());

    data.chunks(4 + 4 * d as usize)
        .map(|row| {
            row.chunks_exact(4)
                .skip(1)
                .map(|e| T::from_le_bytes(e.try_into().unwrap()))
                .collect()
        })
        .collect()
}
