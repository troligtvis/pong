use crate::na;
use rand::{self, thread_rng, Rng};

pub struct Util { }

impl Util {

    pub fn clamp(value: &mut f32, low: f32, high: f32) {
        if *value < low {
            *value = low;
        } else if *value > high {
            *value = high;
        }
    }

    pub fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
        let mut rng = thread_rng();
        vec.x = match rng.gen_bool(0.5) {
            true => x,
            false => -x,
        };

        vec.y = match rng.gen_bool(0.5) {
            true => y,
            false => -y,
        };
    }
}