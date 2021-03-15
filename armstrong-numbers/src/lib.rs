pub fn is_armstrong_number(num: u32) -> bool {
    let input = num.to_string();
    let exp = input.len() as u32;
    let mut result = 0;

    for digit in input.chars() {
        result += digit.to_string().parse::<u32>().unwrap().pow(exp);
        println!("{} {} {}", digit, exp, 5_u32.pow(1));
    }

    num == result
}
