use physim::*;

fn main() {
    let mut timer = Timer::new();
    let mut obj = Mover::new();
    obj.set_velocity(100.0, 100.0, 0.0);
    obj.set_accelaration(-1.0, -0.1, 0.0);
    timer.register_mover(obj);
    timer.run();
}
