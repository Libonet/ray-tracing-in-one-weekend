use crate::utility::vec3::Vec3;

use super::interval::Interval;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, f: &mut std::fmt::Formatter<'_>) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Translate from [0,1] to [0,255]
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256. * intensity.clamp(r)) as i32;
        let gbyte = (256. * intensity.clamp(g)) as i32;
        let bbyte = (256. * intensity.clamp(b)) as i32;

        // Write to formatter
        _ = writeln!(f, "{} {} {}", rbyte, gbyte, bbyte);
    }
}
