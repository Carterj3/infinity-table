use crate::modes::constant::Constant;
use crate::modes::Mode;
use crate::Led;

#[derive(Debug, Clone)]
pub struct Off {
    constant: Constant,
}

impl Off {
    pub fn new(length: usize) -> Off {
        Off {
            constant: Constant::new(length, Led::new(0xE0, 0, 0, 0)),
        }
    }
}

impl Mode for Off {
    fn advance(&mut self) -> Vec<Led> {
        self.constant.advance()
    }
}
