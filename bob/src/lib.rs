pub fn reply(message: &str) -> &str {
    if message.trim_end().ends_with('?')
        && message.to_uppercase() == message
        && message.chars().any(char::is_alphabetic)
    {
        "Calm down, I know what I'm doing!"
    } else if message.to_uppercase() == message
        && !message.trim_end().is_empty()
        && message.chars().any(char::is_alphabetic)
    {
        "Whoa, chill out!"
    } else if message.trim_end().ends_with('?') {
        "Sure."
    } else if message.trim_end().is_empty() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
