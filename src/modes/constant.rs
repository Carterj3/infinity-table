use crate::modes::Mode;
use crate::Led;

#[derive(Debug, Clone)]
pub struct Constant {
    leds: Vec<Led>,
}

impl Constant {
    pub fn new(length: usize, led: Led) -> Constant {
        let leds = {
            let mut leds = Vec::new();
            for _ in 0..length {
                leds.push(led.clone());
            }
            leds
        };

        Constant { leds }
    }
}

impl Mode for Constant {
    fn advance(&mut self) -> Vec<Led> {
        self.leds.clone()
    }
}
