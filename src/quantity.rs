use dimension::Dimension;
use unit::Unit;

#[derive(Debug)]
pub struct Quantity {
    value: f64,
    dimension: Dimension,
}

impl Quantity {
    pub fn new(val: f64, dim: &[i32; 9]) -> Quantity {
        Quantity{ value: val, dimension: Dimension::new(dim) }
    }
    pub fn add(&self, other: Quantity) -> Option<Quantity> {
        if let Some(d) = self.dimension.add(&other.dimension) {
            Some(Quantity::new(self.value+other.value, &d.array()))
        } 
        else { None }
    }
    pub fn sub(&self, other: Quantity) -> Option<Quantity> {
        if let Some(d) = self.dimension.sub(&other.dimension) {
            Some(Quantity::new(self.value-other.value, &d.array()))
        } 
        else { None }
    }
    pub fn mul(&self, other: Quantity) -> Quantity {
        Quantity::new(self.value*other.value, &self.dimension.mul(&other.dimension).array())
    }
    pub fn div(&self, other: Quantity) -> Quantity {
        Quantity::new(self.value/other.value, &self.dimension.div(&other.dimension).array())
    }

    pub fn convert_to(&self, units: &Unit) -> Option<f64> {
        if self.dimension == units.dimension { Some(self.value) }
        else { None }
    }
}
