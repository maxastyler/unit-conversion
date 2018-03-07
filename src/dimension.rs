//! A physical dimension.

#[derive(Debug, PartialEq)]
/// This struct contains the dimension information
pub struct Dimension {
    pub length: i32, //In metres
    pub time: i32, //In seconds
    pub mass: i32, //In kilograms
    pub temperature: i32, //In kelvin
    pub amount: i32, //In moles
    pub current: i32, //In amperes
    pub luminosity: i32, //In candela
    pub plane: i32, //In radians
    pub solid: i32, //In steradians
}

impl Dimension {
    pub fn new(dimensions: &[i32; 9]) -> Dimension {
        Dimension{
            length: dimensions[0],
            time: dimensions[1], 
            mass: dimensions[2], 
            temperature: dimensions[3],
            amount: dimensions[4],
            current: dimensions[5],
            luminosity: dimensions[6],
            plane: dimensions[7], 
            solid: dimensions[8], 
        }
    }
    pub fn array(&self) -> [i32; 9] {
        [
            self.length,
            self.time,
            self.mass,
            self.temperature,
            self.amount,
            self.current,
            self.luminosity,
            self.plane,
            self.solid
        ]
    }
    pub fn matches(&self, other: &Dimension) -> bool {
        if self.array() == other.array() { true }
        else { false }
    }
    pub fn add(&self, other: &Dimension) -> Option<Dimension> {
        if self.matches(other) {
            Some(Dimension::new(&[
                self.length,
                self.time,
                self.mass,
                self.temperature,
                self.amount,
                self.current,
                self.luminosity,
                self.plane,
                self.solid,
            ]))
        }
        else { None }
    }
    pub fn sub(&self, other: &Dimension) -> Option<Dimension> {
        self.add(other)
    }
    pub fn invert(&self) -> Dimension {
        let a: Vec<i32> = self.array().iter().map(|x| -x).collect();
        Dimension::new(&[a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]])
    }
    pub fn mul(&self, other: &Dimension) -> Dimension {
        let a: Vec<i32> = self.array().iter().zip(other.array().iter()).map(|(x, y)| x+y).collect();
        Dimension::new(&[a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]])
    }
    pub fn div(&self, other: &Dimension) -> Dimension {
        self.mul(&other.invert())
    }
    pub fn pow(&self, power: i32) -> Dimension {
        let a: Vec<i32> = self.array().iter().map(|x| x*power).collect();
        Dimension::new(&[a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]])
    }
    pub fn root(&self, root: i32) -> Dimension {
        let a: Vec<i32> = self.array().iter().map(|x| x/root).collect();
        Dimension::new(&[a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]])
    }
}
