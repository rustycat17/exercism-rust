pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res_vec = vec![];
    let mut result = 0;

    for i in 1..limit {
        for k in factors {
            if *k > 0 {
                if i % k == 0 && !res_vec.contains(&i) {
                    res_vec.push(i);
                }
            }
        }
    }

    for element in res_vec {
        result += element;
    }

    result
}
