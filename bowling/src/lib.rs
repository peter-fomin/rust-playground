#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingScore {
    pub score: u16,
    extra_rolls: u16,
}

impl BowlingScore {
    pub fn new() -> Self {
        Self {
            score: 0,
            extra_rolls: 0,
        }
    }

    pub fn add_extra_score(&mut self, rolls: u16) {
        // after a strike or a spare we score extra for next rolls
        self.extra_rolls += rolls;
    }

    pub fn add_pins(&mut self, pins: u16) {
        match &self.extra_rolls {
            3 => {
                self.score += 3 * pins;
                self.extra_rolls -= 2;
            }
            2 | 1 => {
                self.score += 2 * pins;
                self.extra_rolls -= 1;
            }
            _ => self.score += pins,
        }
    }
}

pub struct BowlingGame {
    score: BowlingScore,
    frames: u16,
    throws: u16,
    pins: u16,
    fill_balls: u16,
    is_finished: bool,
}

impl BowlingGame {
    const NUM_FRAMES: u16 = 10;
    const NUM_PINS: u16 = 10;

    pub fn new() -> Self {
        Self {
            score: BowlingScore::new(),
            frames: 0,
            throws: 0,
            pins: Self::NUM_PINS,
            fill_balls: 0,
            is_finished: false,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_finished {
            Some(self.score.score)
        } else {
            None
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.is_finished {
            return Err(Error::GameComplete);
        } else if self.frames < Self::NUM_FRAMES {
            // Regular game
            self.score.add_pins(pins);
            if pins == Self::NUM_PINS {
                // strike
                if self.frames < Self::NUM_FRAMES - 1 {
                    // fill balls instead of extra score in the last frame
                    self.score.add_extra_score(2);
                } else {
                    self.fill_balls = 2;
                }
                self.new_frame();
            } else if self.pins == pins {
                // spare
                if self.frames < Self::NUM_FRAMES - 1 {
                    // fill balls instead of extra score in the last frame
                    self.score.add_extra_score(1);
                } else {
                    self.fill_balls = 1;
                }
                self.new_frame();
            } else if self.throws == 1 {
                // next frame
                self.new_frame();
                if self.frames == Self::NUM_FRAMES {
                    self.is_finished = true;
                }
            } else {
                // next throw
                self.pins -= pins;
                self.throws += 1;
            }
        } else if self.frames == Self::NUM_FRAMES && self.fill_balls > 0 {
            // Fill balls
            self.score.add_pins(pins);
            if self.fill_balls == 2 && pins < Self::NUM_PINS {
                // if first fill ball did not strike
                self.pins -= pins;
            }
            self.fill_balls -= 1;
            if self.fill_balls == 0 {
                self.is_finished = true;
            }
        }
        Ok(())
    }

    fn new_frame(&mut self) {
        self.frames += 1;
        self.pins = Self::NUM_PINS;
        self.throws = 0;
    }
}
