pub mod chaser;
pub mod constant;
pub mod off;
pub mod wave;

use ::std::fmt::Debug;

use crate::Led;

pub trait Mode: Send + Sync + Debug {
    fn advance(&mut self) -> Vec<Led>;
}
