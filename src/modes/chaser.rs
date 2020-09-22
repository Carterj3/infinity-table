use crate::modes::Mode;
use crate::Led;

#[derive(Debug, Clone)]
pub struct Chaser<M> {
    mode: M,
    counter: usize,
}

impl<M> Chaser<M>
where
    M: Mode,
{
    pub fn new(mode: M) -> Chaser<M> {
        Chaser { mode, counter: 0 }
    }
}

impl<M> Mode for Chaser<M>
where
    M: Mode,
{
    fn advance(&mut self) -> Vec<Led> {
        let mut leds = self.mode.advance();

        if let Some(led) = leds.get_mut(self.counter) {
            led.rotate();
        }

        if self.counter >= leds.len() {
            self.counter = 0;
        } else {
            self.counter = self.counter + 1;
        }

        leds
    }
}
