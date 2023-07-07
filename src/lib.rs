use core::fmt;
use std::ops::{Mul, Add, Div, Sub};
use std::time::{Duration, Instant};
use std::fmt::{Display, Formatter};

pub fn greet() {
    println!("Hello!");
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn with(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn x(&mut self, x: f64) -> &mut Self { self.x = x; self }
    pub fn y(&mut self, y: f64) -> &mut Self { self.y = y; self }
    pub fn z(&mut self, z: f64) -> &mut Self { self.z = z; self }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self { 
            x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}, {:.3}, {:.3}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
pub struct Mover {
    location: Point,
    weight: f64,
    velocity: Point,
    acceleration: Point,
    force: Vec<Force>,
    sumf: Point,
}

impl Display for Mover {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"----------------------------------------------
Loc: {}
Vel: {}
Acc: {}
Frc: {}
"#, self.location, self.velocity, self.acceleration, self.sumf)
    }
}

impl Mover {
    pub fn new() -> Self {
        Mover {
            location: Point::new(),
            weight: 0.0,
            velocity: Point::new(),
            acceleration: Point::new(),
            force: Vec::new(),
            sumf: Point::new(),
        }
    }
    pub fn set_location(&mut self, x: f64, y: f64, z: f64) {
        self.location.set(x, y, z);
    }
    pub fn set_accelaration(&mut self, x: f64, y: f64, z: f64) {
        self.acceleration.set(x, y, z);
    }
    pub fn set_velocity(&mut self, x: f64, y: f64, z: f64) {
        self.velocity.set(x, y, z);
    }
    pub fn set_force(&mut self, id: String, x: f64, y: f64, z: f64) {
        let f = Point::with(x, y, z);
        self.force.push(Force {id, f});
        self.sumf = self.sumf + f;
    }
    pub fn drop_force(&mut self, id: String) {
        for i in 0..self.force.len() {
            if self.force[i].id == id {
                self.sumf = self.sumf - self.force[i].f;
                self.force.remove(i);
                break;
            }
        }
    }
    pub fn set_weight(&mut self, w: f64) {
        self.weight = w;
    }
    pub fn print_location(&self) {
        println!("{:?}", self.location);
    }
    pub fn force_sum(&self) -> Point {
        let mut p = Point::new();
        for f in self.force.iter() {
            p = p + f.f;
        }
        p
    }
}

pub trait Movable {
    fn tick(&mut self, dt: f64);
}

impl Movable for Mover {
    fn tick(&mut self, dt: f64) {
        self.acceleration = self.sumf / self.weight;
        self.velocity = 
            self.location + 
            self.velocity * dt + 
            self.acceleration * dt.powi(2) * 0.5;
        self.location = self.acceleration * dt + self.velocity;
        
    }
}

#[derive(Debug, Clone)]
struct Force {
    id: String,
    f: Point
}


pub struct Timer {
    min_duration: Duration,
    start: Instant,
    last_instant: Instant,
    movers: Vec<Mover>,
    print_location: bool,
    print_dt: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            min_duration: Duration::from_millis(7),
            start: Instant::now(),
            last_instant: Instant::now(),
            movers: Vec::new(),
            print_location: true,
            print_dt: true,
        }
    }

    /// Set the minimum tick duration dur = 1000 / max_fps
    pub fn set_min_duration(&mut self, dur: u64) {
        self.min_duration = Duration::from_millis(dur);
    }

    pub fn register_mover(&mut self, mov: Mover) {
        self.movers.push(mov);
    }


    pub fn run(&mut self) {
        self.last_instant = Instant::now();
        self.start = self.last_instant;
        loop {
            let ins = Instant::now();
            let dur = ins - self.last_instant;
            if dur >= self.min_duration {
                self.last_instant = ins;
                let dt = dur.as_secs_f64();


                // Movement
                for mv in self.movers.iter_mut() {
                    mv.tick(dt);
                    if self.print_location {
                        // mv.print_location();
                        println!("{}", mv);
                    }
                    if self.print_dt {
                        println!("dt: {:?} s", dt);
                    }
                }
            }
        }
    }
}
