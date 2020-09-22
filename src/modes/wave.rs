use crate::modes::Mode;
use crate::Led;

#[derive(Debug, Clone)]
pub struct Wave {
    length: usize,
    counter: usize,
    brightness: u8,
}

impl Wave {
    pub fn new(length: usize, brightness: u8) -> Wave {
        Wave {
            length: length,
            counter: 0,
            brightness: brightness,
        }
    }
}

impl Mode for Wave {
    fn advance(&mut self) -> Vec<Led> {
        let mut leds = Vec::new();

        for i in 0..self.length {
            let f = (i as f32) + (self.counter as f32);

            let red = 127.0 * f.to_radians().sin() + 128.0;
            let green = 127.0 * f.to_radians().cos() + 128.0;
            let blue = 127.0 * (180.0 + f).to_radians().sin() + 128.0;

            leds.push(Led::new(
                self.brightness,
                red as u8,
                green as u8,
                blue as u8,
            ));
        }

        if self.counter >= (360 * 360) {
            self.counter = 0;
        } else {
            self.counter = self.counter + 1;
        }

        leds
    }
}
