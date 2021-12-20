pub fn to_usize(slice: &[u8]) -> usize {
    slice.iter().fold(0, |acc, &b| acc*2 + b as usize)
}

pub fn to_u8(slice: &[u8]) -> u8 {
    slice.iter().fold(0, |acc, &b| acc*2 + b)
}