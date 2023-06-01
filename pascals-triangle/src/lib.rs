pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut v = Vec::<Vec<u32>>::new();
        if row_count >= 1 {
            v.push(vec![1]);
        }
        for row in 2..=row_count {
            let lst = v.last().unwrap();
            let mut cur = Vec::<u32>::new();
            cur.push(1);
            for col in 2..=row-1 {
                cur.push(lst[(col - 2) as usize] + lst[(col - 1) as usize]);
            }
            cur.push(1);
            v.push(cur);
        }
        PascalsTriangle(v)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
