//! Units file

use dimension::Dimension;
use std::collections::HashMap;
use std::ops::{Mul, Div};
use parser::get_units_from_lines;

#[derive(Debug, PartialEq)]
pub struct Unit{
    pub value: f64,
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
}
pub fn build_unit(unit_map: HashMap<&str, Unit>, comps: Vec<(&str, i32)> ) -> Option<Unit> {
    let mut cur_unit = Some(Unit::identity());
    for (u_s, i) in comps.into_iter() {
        match unit_map.get(u_s) {
            Some(u) => { for j in 0..i.abs() {
                if i>=0 { cur_unit = Some(cur_unit.unwrap().mul(u)); }
                else { cur_unit = Some(cur_unit.unwrap().div(u)); }
                }},
            None => { cur_unit = None;
                break
            }
        }
    }
    cur_unit
}

pub fn units_from_str(unit_map: &mut HashMap<&str, Unit>, unit_string: &str) {
    
}
