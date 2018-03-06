use dimension::Dimension;
use std::ops::{Mul, Div};

#[derive(Debug)]
pub struct Unit{
    value: f64,
    dimension: Dimension,
}

impl Unit {
    pub fn new(val: f64, dim: &[i32; 9]) -> Unit {
        Unit { value: val, dimension: Dimension::new(dim) }
    }
}

impl<'a> Mul<&'a Unit> for Unit {
    type Output = Self;
    fn mul(self, other: &'a Unit) -> Unit {
        Unit::new(self.value*other.value, &self.dimension.mul(&other.dimension).array())
    }
}


impl<'a> Div<&'a Unit> for Unit {
    type Output = Self;
    fn div(self, other: &'a Unit) -> Unit {
        Unit::new(self.value/other.value, &self.dimension.div(&other.dimension).array())
    }
}
