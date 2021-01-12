#[derive(Debug)]
pub struct HighScores <'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self{ scores: scores }       
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(t) => Some(*t),
            _ => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        match v.first() {
            Some(t) => Some(*t),
            _ => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        if v.len() < 3 {
            return v.drain(0..v.len()).collect()
        }
        v.drain(0..3).collect()
    }
}
