#[derive(Debug)]
pub struct Dimension {
    length: i32, //In metres
    time: i32, //In seconds
    mass: i32, //In kilograms
    temperature: i32, //In kelvin
    amount: i32, //In moles
    current: i32, //In amperes
    luminosity: i32, //In candela
    plane: i32, //In radians
    solid: i32, //In steradians
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
        self.invert().mul(other)
    }
}
