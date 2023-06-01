#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(Vec::from(scores))
    }

    pub fn scores(&self) -> &[u32] {
        self.0.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut tmp = self.0.clone();
        tmp.sort();
        tmp.last().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut tmp = self.0.clone();
        tmp.sort_by(|a, b| b.cmp(a));
        tmp.truncate(3);
        tmp
    }
}
