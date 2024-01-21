const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}
fn main() {
    let mut speed: i32 = 10000;
    speed = clamp_speed(speed);
    println!("Speed is {:?}", speed);
}