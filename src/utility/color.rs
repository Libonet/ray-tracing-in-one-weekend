use crate::utility::vec3::Vec3;

use super::{interval::Interval, vec3::Precision};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: Precision) -> Precision {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    0.
}

impl Color {
    pub fn write_color(&self) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // Translate from [0,1] to [0,255]
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256. * intensity.clamp(r)) as i32;
        let gbyte = (256. * intensity.clamp(g)) as i32;
        let bbyte = (256. * intensity.clamp(b)) as i32;

        println!("{} {} {}", rbyte, gbyte, bbyte);
    }
}
