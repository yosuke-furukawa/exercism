pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<Vec<char>> = vec![];
    for line in minefield {
        board.push(line.chars().collect());
    }

    let mut result: Vec<String> = vec![];
    for (x, b) in board.iter().enumerate() {
        let mut res = String::new();
        for (y, c) in b.iter().enumerate() {
            let x: i32 = x as i32;
            let y: i32 = y as i32;
            let tx = &[x - 1, x, x + 1];
            let ty = &[y - 1, y, y + 1];
            let mut minecount = 0;
            if c == &' ' {
                for px in tx {
                    if px < &0 || px >= &(board.len() as i32) {
                        continue;
                    }
                    for py in ty {
                        if *px == x && *py == y {
                            continue;
                        }
                        if py < &0 || py >= &(board[*px as usize].len() as i32) {
                            continue;
                        }
                        if board[*px as usize][*py as usize] == '*' {
                            minecount += 1;
                        }
                    }
                }
            }
            let r = match (minecount, c) {
                (0, ' ') => " ".to_string(),
                (x, ' ') => x.to_string(),
                (_, _) => "*".to_string(),
            };
            res += &r;
        }
        result.push(res);
    }
    result
}
