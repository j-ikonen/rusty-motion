use std::ops::{Mul, Add};
use std::time::{Duration, Instant};

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

#[derive(Debug)]
pub struct Mover {
    location: Point,
    weight: f64,
    velocity: Point,
    acceleration: Point,
    force: Point,
}

impl Mover {
    pub fn new() -> Self {
        Mover {
            location: Point::new(),
            weight: 0.0,
            velocity: Point::new(),
            acceleration: Point::new(),
            force: Point::new(),
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
    pub fn set_force(&mut self, x: f64, y: f64, z: f64) {
        self.force.set(x, y, z);
    }
    pub fn set_weight(&mut self, w: f64) {
        self.weight = w;
    }
    pub fn print_location(&self) {
        println!("{:?}", self.location);
    }
}

pub trait Movable {
    fn tick(&mut self, dt: f64);
}

impl Movable for Mover {
    fn tick(&mut self, dt: f64) {
        self.velocity = 
            self.location + 
            self.velocity * dt + 
            self.acceleration * dt.powi(2) * 0.5;
        self.location = self.acceleration * dt + self.velocity;
        
    }
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
                for mv in self.movers.iter_mut() {
                    mv.tick(dt);
                    if self.print_location {
                        mv.print_location();
                    }
                    if self.print_dt {
                        println!("dt: {:?} s", dt);
                    }
                }
            }
        }
    }
}
