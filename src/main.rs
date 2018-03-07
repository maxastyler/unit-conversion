/// This program converts between different physical units

#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

mod dimension;
mod quantity;
mod unit;
mod parser;

use unit::Unit;

fn read_file_to_str(file_path: &str) -> Result<String, io::Error> {
    let mut file = try!(File::open(file_path));
    let mut f_s = String::new();
    try!(file.read_to_string(&mut f_s));
    Ok(f_s)
}

fn main() { 
    let s = read_file_to_str("standard_units").unwrap();
    let mut units: HashMap<&str, Unit> = HashMap::new();
    units.insert("identity", Unit::identity());
    let a = quantity::Quantity::new(10.0, &[2, 0, 0, 0, 0, 0, 0, 0, 0]);
    let u = unit::build_unit(& units, vec!(("milli", 2), ("metre", 2)));
    unit::units_from_str(&mut units, &s);
    println!("{:?}", units);
}
