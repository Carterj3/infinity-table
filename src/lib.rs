pub mod modes;

#[derive(Debug, Clone)]
pub struct Led {
    brightness: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl Led {
    pub fn new(brightness: u8, red: u8, green: u8, blue: u8) -> Led {
        Led {
            brightness,
            red,
            green,
            blue,
        }
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        [
            self.brightness.clone(),
            self.blue.clone(),
            self.green.clone(),
            self.red.clone(),
        ]
    }

    fn rotate(&mut self) {
        let r = self.red;
        let g = self.green;
        let b = self.blue;

        self.red = g;
        self.green = b;
        self.blue = r;
    }
}
