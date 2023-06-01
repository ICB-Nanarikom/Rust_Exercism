#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    cnt_frame: u16,
    cnt_throw: u16,
    cnt_pin: u16,
    cnt_2: u16,
    cnt_1: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            score: 0,
            cnt_frame: 1, 
            cnt_throw: 1,
            cnt_pin: 10,
            cnt_2: 0,
            cnt_1: 0,
        }
    }

    fn new_frame(&mut self) {
        self.cnt_frame += 1;
        self.cnt_throw = 1;
        self.cnt_pin = 10;
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.cnt_2 == 0 && self.cnt_1 == 0 && self.cnt_frame > 10 {
            return Result::Err(Error::GameComplete);
        }
        if self.cnt_pin < pins {
            return Result::Err(Error::NotEnoughPinsLeft);
        }
        
        if self.cnt_frame <= 10 { self.score += pins; }
        self.score += pins * (self.cnt_2 + self.cnt_1);
        self.cnt_pin -= pins;
        self.cnt_1 = self.cnt_2;
        self.cnt_2 = 0;

        if self.cnt_pin == 0 {
            match self.cnt_throw {
                1 => {
                    if self.cnt_frame <= 10 { self.cnt_2 += 1; }
                    self.new_frame();
                },
                2 => {
                    if self.cnt_frame <= 10 { self.cnt_1 += 1; }
                    self.new_frame();
                },
                _ => {},
            }
        } else {
            match self.cnt_throw {
                1 => {
                    self.cnt_throw += 1;
                },
                2 => {
                    self.new_frame();
                },
                _ => {},
            }
        }

        Result::Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.cnt_2 == 0 && self.cnt_1 == 0 && self.cnt_frame > 10 { Some(self.score) } else { None }
    }
}
