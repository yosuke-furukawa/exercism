pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_rows = vec![];
    for item in input {
        let mut max = 0;
        for n in item {
            max = max.max(*n);
        }
        max_rows.push(max);
    }

    let mut min_cols = vec![];
    for row in 0..input[0].len() {
        let mut min = std::u64::MAX;
        for col in input {
            min = min.min(col[row]);
        }
        min_cols.push(min);
    }

    let mut result: Vec<(usize, usize)> = vec![];

    for col in 0..input.len() {
        for row in 0..input[0].len() {
            if input[col][row] == max_rows[col] && input[col][row] == min_cols[row] {
                result.push((col, row));
            }
        }
    }
    result
}
