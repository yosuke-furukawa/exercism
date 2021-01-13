const SURE: &str = "Sure.";
const WHOA: &str = "Whoa, chill out!";
const CALM: &str = "Calm down, I know what I'm doing!";
const FINE: &str = "Fine. Be that way!";
const WHAT: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let mem = message.to_string();
    let say_nothing = mem.trim() == "";
    let has_alphabet = mem.chars().any(|x| x.is_alphabetic());
    let is_all_capital = has_alphabet && mem.to_uppercase() == mem;
    let is_question = mem.trim().chars().last().unwrap_or('\0') == '?';
    match (say_nothing, is_all_capital, is_question) {
        (_, false, true) => SURE,
        (_, true, true) => CALM,
        (_, true, false) => WHOA,
        (true, _, _) => FINE,
        (_, _, _) => WHAT,
    }
}
