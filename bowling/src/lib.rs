#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum FrameResult {
    Open,
    Strike,
    StrikeSeq,
    Spare,
}

#[derive(PartialEq)]
pub enum GameState {
    Regular,
    FillBall,
    TwoFillBalls,
    Finished,
}

pub struct BowlingGame {
    score: u16,
    frame: u16,
    throw: u16,
    pins: u16,
    prev_frame: FrameResult,
    state: GameState,
}

impl BowlingGame {
    const NUM_FRAMES: u16 = 10;
    const NUM_PINS: u16 = 10;

    pub fn new() -> Self {
        Self {
            score: 0,
            frame: 1,
            throw: 1,
            pins: Self::NUM_PINS,
            prev_frame: FrameResult::Open,
            state: GameState::Regular,
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.state {
            GameState::Finished => Some(self.score),
            _ => None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        } else if let GameState::Finished = self.state {
            return Err(Error::GameComplete);
        } else {
            self.roll_ball(pins);
        }
        Ok(())
    }

    fn roll_ball(&mut self, pins: u16) {
        self.pins -= pins;
        self.advance_score(pins);

        match &self.state {
            GameState::Regular => {
                if self.pins == 0 || self.throw == 2 { // end of a frame
                    self.prev_frame = if self.throw == 1 { // strike
                        match self.prev_frame {
                            FrameResult::Strike | FrameResult::StrikeSeq => FrameResult::StrikeSeq,
                            _ => FrameResult::Strike,
                        }
                    } else if self.pins == 0 { // spare
                        FrameResult::Spare
                    } else { //open
                        FrameResult::Open
                    };
                    if self.frame == Self::NUM_FRAMES { // end of game frames
                        self.state = match self.prev_frame {
                            FrameResult::Strike | FrameResult::StrikeSeq => GameState::TwoFillBalls,
                            FrameResult::Spare => GameState::FillBall,
                            _ => GameState::Finished,
                        };
                    }
                    self.frame += 1;
                    self.throw = 1;
                    self.pins = Self::NUM_PINS;
                } else { // not end of a frame
                    self.throw += 1;
                }
            }
            GameState::FillBall => self.state = GameState::Finished,
            GameState::TwoFillBalls => {
                if self.throw == 2 {
                    self.state = GameState::Finished;
                } else {
                    self.state = GameState::FillBall;
                    if pins == Self::NUM_PINS {
                        self.pins = Self::NUM_PINS;
                    }
                    self.throw += 1;
                }
            }
            GameState::Finished => (),
        }
    }

    fn advance_score(&mut self, pins: u16) {
        match (&self.state, &self.prev_frame, &self.throw) {
            (GameState::Regular, FrameResult::StrikeSeq, 1) => self.score += 3 * pins,
            (GameState::Regular, FrameResult::StrikeSeq, 2)
            | (GameState::Regular, FrameResult::Strike, _)
            | (GameState::Regular, FrameResult::Spare, 1)
            | (GameState::TwoFillBalls, FrameResult::StrikeSeq, _) => self.score += 2 * pins,
            _ => self.score += pins,
        }
    }
}
