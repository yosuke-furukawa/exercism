pub struct PascalsTriangle {
    row_count: usize
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{ row_count: row_count as usize }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::with_capacity(self.row_count);

        for i in 0..self.row_count {
            match i {
                0 => rows.push(vec![1]),
                n => {
                    let mut row = vec![1];
                    for values in rows[n-1].windows(2) {
                        row.push(values[0] + values[1]);
                    }
                    row.push(1);
                    rows.push(row);
                },
            }
        }
        rows
    }
}
