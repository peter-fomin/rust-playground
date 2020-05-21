#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut problem = match start_bucket {
        Bucket::One => BucketProblem::new(capacity_1, capacity_2, goal),
        Bucket::Two => BucketProblem::new(capacity_2, capacity_1, goal),
    };
    problem.solve();
    problem.get_answer(start_bucket)
}

pub struct BucketProblem {
    capacity_1: u8,
    capacity_2: u8,
    contents_1: u8,
    contents_2: u8,
    moves: u8,
    goal: u8,
}

impl BucketProblem {
    pub fn new(capacity_1: u8, capacity_2: u8, goal: u8) -> Self {
        // The BucketProblem just assumes that we fill first the first bucket
        // (which is kinda logical since we can just switch the function arguments)
        Self {
            capacity_1,
            capacity_2,
            contents_1: capacity_1,
            contents_2: 0,
            moves: 1,
            goal,
        }
    }

    pub fn get_answer(&self, start_bucket: &Bucket) -> Option<BucketStats> {
        let (goal_bucket, other_bucket) = match (
            self.contents_1 == self.goal,
            self.contents_2 == self.goal,
            start_bucket,
        ) {
            (true, _, Bucket::One) => (Bucket::One, self.contents_2),
            (_, true, Bucket::One) => (Bucket::Two, self.contents_1),
            (true, _, Bucket::Two) => (Bucket::Two, self.contents_2),
            (_, true, Bucket::Two) => (Bucket::One, self.contents_1),
            _ => return None,
        };
        Some(BucketStats {
            moves: self.moves,
            goal_bucket,
            other_bucket,
        })
    }

    pub fn solve(&mut self) {
        if self.capacity_2 == self.goal {
            self.contents_2 = self.capacity_2;
            self.moves += 1;
            return;
        }

        loop {
            if self.contents_1 == self.goal || self.contents_2 == self.goal {
                return;
            }

            if self.contents_1 == 0 {
                self.fill();
            } else if self.contents_2 == self.capacity_2 {
                self.empty();
            } else {
                self.transfer_first_second();
            }
            self.moves += 1;

            if self.contents_1 == self.capacity_1 && self.contents_2 == self.capacity_2 {
                return;
            }
        }
    }

    fn fill(&mut self) {
        self.contents_1 = self.capacity_1;
    }

    fn empty(&mut self) {
        self.contents_2 = 0;
    }

    fn transfer_first_second(&mut self) {
        if self.contents_1 + self.contents_2 > self.capacity_2 {
            self.contents_1 -= self.capacity_2 - self.contents_2;
            self.contents_2 = self.capacity_2;
        } else {
            self.contents_2 += self.contents_1;
            self.contents_1 = 0;
        }
    }
}
