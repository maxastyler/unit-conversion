/// This program converts between different physical units

use std::collections::HashMap;

mod dimension;
mod quantity;
mod unit;

use unit::Unit;

fn main() { 
    let mut units: HashMap<&str, Unit> = HashMap::new();
    units.insert("metre", Unit::new(1.0, &[1, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("second", Unit::new(1.0, &[0, 1, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("kilogram", Unit::new(1.0, &[0, 0, 1, 0, 0, 0, 0, 0, 0]));
    units.insert("kelvin", Unit::new(1.0, &[0, 0, 0, 1, 0, 0, 0, 0, 0]));
    units.insert("mole", Unit::new(1.0, &[0, 0, 0, 0, 1, 0, 0, 0, 0]));
    units.insert("ampere", Unit::new(1.0, &[0, 0, 0, 0, 0, 1, 0, 0, 0]));
    units.insert("candela", Unit::new(1.0, &[0, 0, 0, 0, 0, 0, 1, 0, 0]));
    units.insert("radian", Unit::new(1.0, &[0, 0, 0, 0, 0, 0, 0, 1, 0]));
    units.insert("steradian", Unit::new(1.0, &[0, 0, 0, 0, 0, 0, 0, 0, 1]));
    units.insert("identity", Unit::identity());
    units.insert("kilo", Unit::new(1e3, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("mega", Unit::new(1e6, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("giga", Unit::new(1e9, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("milli", Unit::new(1e-3, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("micro", Unit::new(1e-6, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("nano", Unit::new(1e-9, &[0, 0, 0, 0, 0, 0, 0, 0, 0]));
    units.insert("joule", Unit::new(1.0, &[2, -2, 1, 0, 0, 0, 0, 0, 0]));
    let a = quantity::Quantity::new(10.0, &[2, 0, 0, 0, 0, 0, 0, 0, 0]);
    let u = unit::build_unit(units, vec!(("joule", 1), ("kilo", 1), ("metre", 2)));
    println!("{:?}", u);
}
