use physim::*;

fn main() {
    let max_fps = 142;
    let mut timer = Timer::new();
    let mut obj = Mover::new();
    obj.set_velocity(100.0, 100.0, 0.0);
    obj.set_force("g".to_string(), -1.0, -0.1, 0.0);
    obj.set_weight(1.0);
    timer.register_mover(obj);
    timer.set_min_duration(1000 / max_fps);
    timer.run();
}
