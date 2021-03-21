pub fn raindrops(n: u32) -> String {
    let result = match (n % 3 == 0, n % 5 == 0, n % 7 == 0) {
        (true, false, false) => String::from("Pling"),
        (false, true, false) => String::from("Plang"),
        (false, false, true) => String::from("Plong"),
        (true, true, false) => String::from("PlingPlang"),
        (true, false, true) => String::from("PlingPlong"),
        (false, true, true) => String::from("PlangPlong"),
        (true, true, true) => String::from("PlingPlangPlong"),
        _ => n.to_string()
    };

    result
}
