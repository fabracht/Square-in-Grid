use std::time;
//thread::sleep(ten_millis);
pub const GRAVITY:f64 = -0.2;
pub const DT:f64 = 0.1;

use draw::*;
// s(n+1) = s(n) + v * dt
// v(n+1) = v(n) + G * dt

pub fn jump_mechanics(mut velocity:f64) {
    while &velocity >= &-velocity {
        velocity += GRAVITY * DT;
    }
}