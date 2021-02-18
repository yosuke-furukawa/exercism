use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}
impl Bucket {
    fn other_bucket(&self) -> Bucket {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self { Self { moves, goal_bucket, other_bucket } }
}

pub fn solve(capacity_1: u8,capacity_2: u8,goal: u8,start_bucket: &Bucket,) -> Option<BucketStats> {
    if capacity_1 < goal && capacity_2 < goal {return None;}

    let mut count = 0;
    let mut buckets = (0u8, 0u8);
    let (capacity_a, capacity_b) = match *start_bucket {
        Bucket::One => (capacity_1, capacity_2),
        Bucket::Two => (capacity_2, capacity_1),
    };
    if goal == capacity_b{ // rotating opposite direction
        return Some(BucketStats::new(2, start_bucket.other_bucket(), capacity_a)  );
    }  

    let mut cycle_detection = HashSet::new();
    loop {
        buckets = match buckets {
            (a, _) if a == goal => return Some(BucketStats::new(count, *start_bucket, buckets.1)),
            (_, b) if b == goal => return Some(BucketStats::new(count, start_bucket.other_bucket(), buckets.0)),

            (_, b) if b == capacity_b => (buckets.0, 0), // empty B if full
            (0, _) => (capacity_a, buckets.1),               // fill A if empty

            (_, _) => {  // pour as much from A to B as possible
                if buckets.0 + buckets.1 <= capacity_b {
                    (0, buckets.0 + buckets.1)
                } else {
                    (buckets.0 + buckets.1 - capacity_b, capacity_b)
                }
            }
        };
        if !cycle_detection.insert(buckets) {
            return None;
        }
        count += 1;
    }
}