#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sqrt = (num as f64).sqrt() as u64;
    if sqrt == 1 {
        return Some(Classification::Deficient);
    }
    let mut answer = 1;
    for i in 2..=sqrt {
        if num % i == 0 {
            if num / i != i {
                answer += num / i + i;
            } else {
                answer += i;
            }
        }
    }

    println!("{}", answer);

    Some(match answer {
        _ if answer == num => Classification::Perfect,
        _ if answer > num => Classification::Abundant,
        _ => Classification::Deficient,
    })
}
