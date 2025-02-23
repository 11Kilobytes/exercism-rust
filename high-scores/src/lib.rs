use core::iter::Iterator;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    top_5: [Option<u32>; 5],
    latest: Option<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let mut sorted_scores: Vec<Option<u32>> = scores.iter().map(|&x| Some(x)).collect();
        sorted_scores.sort_by(|a, b| b.cmp(a));
        sorted_scores.resize(5, None);
        Self {
            scores,
            top_5: sorted_scores[..].try_into().unwrap(),
            latest: scores.last().copied(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.latest
    }

    pub fn personal_best(&self) -> Option<u32> {
        *self.top_5.get(0)?
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_5.iter().flatten().map(|x| *x).take(3).collect()
    }
}
