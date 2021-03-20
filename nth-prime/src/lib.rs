pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];

    let mut current_number = primes[0];
    loop {
        if primes.len() as u32 - 1 == n {
            break;
        }
        current_number += 1;
        let mut is_prime = true;
        'inner: for current_prime in primes.iter() {
            if current_number % current_prime == 0 {
                is_prime = false;
                break 'inner;
            }
        }

        if is_prime {
            primes.push(current_number);
        }
    }

    *primes.last().unwrap()
}
