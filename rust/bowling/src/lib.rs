#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: [Option<u16>; 21],
    rounds: usize,
    finished: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{ frames: [None; 21], rounds: 0, finished: false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.finished {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.rounds < 18 {
            if self.rounds % 2 == 1 && self.frames[self.rounds - 1].unwrap() + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            
            self.frames[self.rounds] = Some(pins);
            if pins == 10 && self.rounds % 2 == 0 {
                self.rounds += 2;
            } else {
                self.rounds += 1;
            }
        } else {
            if self.rounds == 18 {
                self.frames[self.rounds] = Some(pins);
                self.rounds += 1;
            } else if self.rounds == 19 {
                let prev_pin = self.frames[18].unwrap();
                if prev_pin < 10 && prev_pin + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if prev_pin + pins < 10 {
                    self.finished = true;
                }
                self.frames[self.rounds] = Some(pins);
                self.rounds += 1;
            } else {
                let pin18 = self.frames[18].unwrap();
                let pin19 = self.frames[19].unwrap();
                if pin18 == 10 && pin19 < 10 && pin19 + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.frames[self.rounds] = Some(pins);
                self.finished = true;
            }
            
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.finished {
            return None;
        }
        let mut score = 0;
        for i in 0..9 {
            let index1 = i * 2;
            let index2 = (i+1) * 2;
            let index3 = (i+2) * 2;
            score += match (self.frames[index1], self.frames[index1+1], self.frames[index2], self.frames[index2 + 1], self.frames[index3]) {
                (Some(a), Some(b), Some(c), _, _) if a + b == 10 => a + b + c, // Spare
                (Some(a), _, Some(c), Some(d), _) if a == 10 => a + c + d, // Strike
                (Some(a), _, Some(c), None, Some(e)) if a == 10 => a + c + e, // Strike
                (Some(a), Some(b), _, _, _) => a + b,
                _ => 0,
            };
        }
        score += match(self.frames[18], self.frames[19], self.frames[20]) {
            (Some(a), Some(b), Some(c)) => a + b + c,
            (Some(a), Some(b), _) => a + b,
            (Some(a), _, _) => a,
            _ => 0
        };
        
        Some(score)
    }
}
