pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.ends_with('?');
    let is_uppercase =
        message.to_ascii_uppercase() == message && message.chars().any(char::is_alphabetic);
    match (is_question, is_uppercase) {
        (true, false) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        (false, false) => "Whatever.",
        (false, true) => "Whoa, chill out!",   
    }
}
