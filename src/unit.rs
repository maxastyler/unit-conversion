use dimension::Dimension;
use std::collections::HashMap;
use std::ops::{Mul, Div};

#[derive(Debug)]
pub struct Unit{
    value: f64,
    pub dimension: Dimension,
}

impl Unit {
    pub fn identity() -> Unit {
        Unit {value: 1.0, dimension: Dimension::new(&[0, 0, 0, 0, 0, 0, 0, 0, 0])}
    }
    pub fn new(val: f64, dim: &[i32; 9]) -> Unit {
        Unit { value: val, dimension: Dimension::new(dim) }
    }
    pub fn mul(&self, other: &Unit) -> Unit {
        Unit::new(self.value*other.value, &self.dimension.mul(&other.dimension).array())
    }
    pub fn div(&self, other: &Unit) -> Unit {
        Unit::new(self.value/other.value, &self.dimension.div(&other.dimension).array())
    }
    pub fn pow(&self, power: i32) -> Unit {
        Unit::new(self.value.powf(power as f64), &self.dimension.pow(power).array())
    }
    pub fn root(&self, root: i32) -> Unit {
        Unit::new(self.value.powf(1.0/(root as f64)), &self.dimension.root(root).array())
    }
    pub fn build_unit(unit_map: HashMap<&str, Unit>, comps: Vec<(&str, i32)> )-> Option<Unit> {
        None
    }
}