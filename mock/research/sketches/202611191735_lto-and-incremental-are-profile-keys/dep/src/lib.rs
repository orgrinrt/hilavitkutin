pub fn sum(xs: &[u32]) -> u32 {
    xs.iter().fold(0, |a, x| a.wrapping_add(*x))
}
