use anyhow::{anyhow, Error};
use cluster::PixelColor;
use std::env;

mod cluster;
pub mod k_means;

pub fn get_args() -> Result<(String, usize), Error> {
    let mut args = env::args();

    let path = args
        .nth(1)
        .ok_or_else(|| anyhow!("Expected first arg 'image path'"))?;

    let k = args
        .next()
        .ok_or_else(|| anyhow!("Expected second arg 'k number'"))?
        .parse()?;

    Ok((path, k))
}

trait EuclideanDistance {
    fn euclidean_dist(&self, rhs: &Self) -> f32;
}

impl EuclideanDistance for PixelColor {
    fn euclidean_dist(&self, rhs: &Self) -> f32 {
        let [a_r, a_g, a_b, a_a] = self.0;
        let [b_r, b_g, b_b, b_a] = rhs.0;

        let distance = (a_r as i32 - b_r as i32).pow(2)
            + (a_g as i32 - b_g as i32).pow(2)
            + (a_b as i32 - b_b as i32).pow(2)
            + (a_a as i32 - b_a as i32).pow(2);

        (distance as f32).sqrt()
    }
}
