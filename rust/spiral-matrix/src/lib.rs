pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; size as usize]; size as usize];

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let mut count: u32 = 1;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut i = 0;
    let mut min: i32 = 0;
    let mut max: i32 = size as i32;
    loop {
        if y >= max || grid[y as usize][x as usize] != 0 {
            break;
        }
        grid[y as usize][x as usize] = count;
        match (i, x, y) {
            (0, _, _) if x >= max - 1 => {
                i += 1;
            }
            (1, _, _) if y >= max - 1 => {
                i += 1;
            }
            (2, _, _) if x <= min => {
                i += 1;
                min += 1;
            }
            (3, _, _) if y <= min => {
                i = 0;
                max -= 1;
            }
            _ => {}
        }
        count += 1;
        x += dx[i];
        y += dy[i];
    }
    grid
}
