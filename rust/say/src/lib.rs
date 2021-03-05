fn num_to_message(num: u64) -> String {
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        _ => ""
    }.to_string()
}

fn num_to_digits(num: u64) -> String {
    match num {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => ""
    }.to_string()
}

fn say(num: u64) -> Vec<String> {
    let teens = num % 100;
    let hundreds = (num - teens) / 100;
    let mut messages = vec![];

    if teens >= 10 {
        let one = teens % 10;
        let ten = teens - one;
        if ten < 20 {
            messages.push(num_to_message(teens));
        } else {
            if one > 0 && ten > 0 {
                messages.push(num_to_message(ten) + "-" + num_to_message(one).as_str());
            } else if one > 0 {
                messages.push(num_to_message(one));
            } else if ten > 0 {
                messages.push(num_to_message(ten));
            }
        }
    } else if teens > 0 {
        messages.push(num_to_message(teens));
    }
    if hundreds > 0 {
        messages.push(num_to_message(100));
        messages.push(num_to_message(hundreds));
    }
    messages
}

pub fn encode(n: u64) -> String {
    let mut num = n;
    let mut words = vec![];
    let mut cycles = 0;
    while num > 0 {
        let q = num % 1000;
        if q > 0 {
            if cycles > 0 {
                words.push(num_to_digits(cycles));
            }
            words.append(&mut say(q));
        }
        num /= 1000;
        if num > 0 {
            cycles += 1;
        }
    }
    if words.is_empty() {
        return "zero".to_string();
    }
    words.reverse();
    words.join(" ").to_string()
}
