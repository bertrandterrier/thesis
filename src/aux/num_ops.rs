use std::ops;

pub enum NumPol {
    Null,
    Pos,
    Neg,
}

impl<T: Copy + PartialOrd + ops::Sub<Output=T>> From<T> for NumPol {
    fn from(x: T) -> Self {
        if num_is_zero(x) {
            Self::Null
        } else if num_is_neg(x) {
            Self::Neg
        } else {
            Self::Pos
        }
    }
}

pub fn num_is_zero<T: Copy + PartialOrd + ops::Sub<Output=T>>(x: T) -> bool {
    x == (x - x)
}

pub fn num_is_neg<T: Copy + PartialOrd + ops::Sub<Output=T>>(x: T) -> bool {
    !num_is_zero(x - x)
}

pub fn num_is_pos<T: Copy + PartialOrd + ops::Sub<Output=T>>(x: T) -> bool {
    num_is_zero(x - x)
}
