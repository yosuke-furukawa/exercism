pub fn encode(source: &str) -> String {
    let mut answer = String::new();
    if source.is_empty() {
        return answer;
    }
    let mut count = 1;
    let mut chars = source.chars();

    let mut p = chars.next().unwrap();
    for c in chars {
        if p == c {
            count += 1;
        } else if count > 1 {
            answer += &count.to_string();
            answer += &p.to_string();
            count = 1;
        } else {
            answer += &p.to_string();
        }
        p = c;
    }
    if count > 1 {
        answer += &count.to_string();
        answer += &p.to_string();
    } else {
        answer += &p.to_string();
    }
    answer
}

pub fn decode(source: &str) -> String {
    let mut answer = String::new();
    if source.is_empty() {
        return answer;
    }
    let mut count = 0;
    for c in source.chars() {
        if c.is_numeric() {
            count = count * 10 + c.to_digit(10).unwrap();
        } else {
            if count == 0 {
                answer += &c.to_string();
                continue;
            }
            for _ in 0..count {
                answer += &c.to_string();
            }
            count = 0;
        }
    }
    answer
}
