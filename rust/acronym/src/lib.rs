pub fn abbreviate(phrase: &str) -> String {
    let p = phrase.replace("-", " ").replace("_", " ");

    let mut str = String::new();
    for w in p.split_ascii_whitespace() {
        let mut chars = w.chars();
        str += &chars.next().unwrap().to_ascii_uppercase().to_string();
        for cs in chars.collect::<Vec<char>>().windows(3) {
            if cs[0].is_lowercase() && cs[1].is_uppercase() && cs[2].is_lowercase() {
                str += &cs[1].to_string();
            }
        }
    }
    str
}
