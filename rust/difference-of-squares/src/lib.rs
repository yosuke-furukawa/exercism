pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        sum = sum + i;
    }
    sum*sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        sum = sum + i * i;
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    sum_of_squares(n) - square_of_sum(n)
}
