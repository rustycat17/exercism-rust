pub fn square(s: u32) -> u64 {
    let mut result = 1;

    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }

    for _ in 1..s {
        result *= 2;
    }

    result
}

pub fn total() -> u64 {
    let mut result = 0;
    for i in 1..65 {
        result += square(i);
    }

    result
}
