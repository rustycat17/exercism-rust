pub fn verse(n: u32) -> String {
    match n {
        2 => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n", n, n-1),
        1 => format!("{0} bottle of beer on the wall, {0} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();

    for position in (end..=start).rev() {
        result.push_str(&verse(position));
        if position != end {
            result.push('\n');
        }
    }

    result
}
