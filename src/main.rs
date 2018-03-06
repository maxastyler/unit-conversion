mod dimension;

fn main() {
    let a = dimension::Dimension::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0]);
    let b = dimension::Dimension::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0]);
    println!("{:?}", a);
    println!("{:?}", a.div(&b));
}
