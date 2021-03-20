pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut result = vec![];

    let mut number = n;
    while number > 1 {
        'inner: for div in 2..number + 1 {
            if number % div == 0 {
                result.push(div);
                number /= div;
                break 'inner;
            }
        }
    }

    result
}
