pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count {
            let mut row = vec![1_u32];
            for j in 1..=i {
                row.push(rows[i - 1][j - 1] + *rows[i - 1].get(j).unwrap_or(&0));
            }
            rows.push(row);
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
