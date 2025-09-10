use crate::matrix::Matrix;

pub trait Picture {
    fn draw(&mut self) -> Matrix;
}
