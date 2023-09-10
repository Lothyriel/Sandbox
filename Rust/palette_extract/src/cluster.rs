use image::Rgba;

pub type PixelColor = Rgba<u8>;

pub struct Cluster {
    pub center: PixelColor,
    pub points: Vec<PixelColor>,
}

impl Cluster {
    pub fn new(center: PixelColor) -> Self {
        Self {
            center,
            points: Vec::new(),
        }
    }

    pub fn update_centroid(&mut self) {
        let len = self.points.len() as u32;

        if len == 0 {
            return;
        }

        let [mut r, mut g, mut b, mut a] = [0, 0, 0, 0];

        for point in self.points.iter() {
            let [p_r, p_g, p_b, p_a] = point.0;

            r += p_r as u32;
            g += p_g as u32;
            b += p_b as u32;
            a += p_a as u32;
        }

        self.center = Rgba([
            (r / len) as u8,
            (g / len) as u8,
            (b / len) as u8,
            (a / len) as u8,
        ])
    }
}
