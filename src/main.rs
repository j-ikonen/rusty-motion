use physim::*;

fn main() {
    greet();
    let mut p = Point::new();
    p.x(2.3).y(7.3).z(-1.4);
    println!("{:?}", p);
}
