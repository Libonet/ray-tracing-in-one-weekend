use crate::utility::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, f: &mut std::fmt::Formatter<'_>) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Translate from [0,1] to [0,255]
        let rbyte = (255.999 * r) as i32;
        let gbyte = (255.999 * g) as i32;
        let bbyte = (255.999 * b) as i32;

        // Write to formatter
        _ = writeln!(f, "{} {} {}", rbyte, gbyte, bbyte);
    }
}
