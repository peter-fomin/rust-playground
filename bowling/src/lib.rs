#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum GameFlags {
    Open,
    Strike,
    StrikeTwice,
    Spare,
    FillBall,
    TwoFillBalls,
    TwoFillBallsAndStrike,
    Finished,
}

pub struct BowlingGame {
    score: u16,
    frames: u16,
    throws: u16,
    pins: u16,
    flags: GameFlags,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            score: 0,
            frames: 0,
            throws: 0,
            pins: 10,
            flags: GameFlags::Open,
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.flags {
            GameFlags::Finished => Some(self.score),
            _ => None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        } else if let GameFlags::Finished = self.flags {
            return Err(Error::GameComplete);
        } else if let GameFlags::FillBall = self.flags {
            self.advance_score(pins);
            self.advance_flags(pins);
        } else if let GameFlags::TwoFillBalls = self.flags {
            self.advance_score(pins);
            self.advance_flags(pins);
            if pins < 10 {
                self.pins -= pins;
            }
        } else if let GameFlags::TwoFillBallsAndStrike = self.flags {
            self.advance_score(pins);
            self.advance_flags(pins);
            if pins < 10 {
                self.pins -= pins;
            }
        } else {
            self.roll_ball(pins);
        }

        Ok(())
    }

    fn roll_ball(&mut self, pins: u16) {
        self.throws += 1;
        self.pins -= pins;
        self.advance_score(pins);
        self.advance_flags(pins);

        if self.throws == 2 || self.pins == 0 {
            self.advance_frame()
        }
    }

    fn advance_score(&mut self, pins: u16) {
        match self.flags {
            GameFlags::StrikeTwice => self.score += 3 * pins,
            GameFlags::Strike | GameFlags::Spare | GameFlags::TwoFillBallsAndStrike => self.score += 2 * pins,
            _ => self.score += pins,
        }
    }

    fn advance_flags(&mut self, pins: u16) {
        self.flags = match (&self.flags, pins, self.pins) {
            (GameFlags::StrikeTwice, 10, 0) | (GameFlags::Strike, 10, 0) => GameFlags::StrikeTwice,
            (_, 10, 0) => GameFlags::Strike,
            (GameFlags::Strike, _, _) | (GameFlags::StrikeTwice, _, _) | (_, _, 0) => GameFlags::Spare,
            (GameFlags::TwoFillBalls, _, _) | (GameFlags::TwoFillBallsAndStrike, _, _) => GameFlags::FillBall,
            (GameFlags::FillBall, _, _) => GameFlags::Finished,
            _ => GameFlags::Open,
        }

    }

    fn advance_frame(&mut self) {
        self.pins = 10;
        self.frames += 1;
        self.throws = 0;
        match (self.frames, &self.flags) {
            (10, GameFlags::StrikeTwice) => self.flags = GameFlags::TwoFillBallsAndStrike,
            (10, GameFlags::Strike) => self.flags = GameFlags::TwoFillBalls,
            (10, GameFlags::Spare) => self.flags = GameFlags::FillBall,
            (10, _) => self.flags = GameFlags::Finished,
            _ => (),
        }
    }
}
