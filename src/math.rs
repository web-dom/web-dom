extern "C" {
    fn math_random() -> f32;
    fn math_random_n(n: i32) -> i32;
}

pub fn random() -> f32 {
    unsafe { math_random() }
}

pub fn random_n(n: i32) -> i32 {
    unsafe { math_random_n(n) }
}
