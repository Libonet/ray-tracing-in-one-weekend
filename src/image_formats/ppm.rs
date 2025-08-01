use crate::utility::{color::Color, vec3::Precision};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PPM {
    cols: usize,
    rows: usize,
    max_color: u8,
    values: Vec<Color>,
}

impl PPM {
    pub fn new(cols: usize, rows: usize, max_color: u8, values: Vec<Color>) -> Self {
        assert!(values.len() == cols * rows);
        Self { cols, rows, max_color, values }
    }

    pub fn generate<F>(cols: usize, rows: usize, max_color: u8, gen: F) -> Self
    where 
        F: Fn(Precision, Precision) -> Color,
    {
        let mut values = Vec::with_capacity(cols * rows);

        for row in 0..rows {
            eprintln!("Scanlines remaining: {}", rows - row);
            for col in 0..cols {
                values.push(gen(row as Precision, col as Precision));
            }
        }

        eprintln!("Done! :D");

        PPM::new(cols, rows, max_color, values)
    }

    pub fn output(&self) {
        println!("P3");
        println!("{} {}", self.cols, self.rows);
        println!("{}", self.max_color);
        
        for row in 0..self.rows {
            for col in 0..self.cols {
                let rgb = self.values[row * self.cols + col];
                rgb.write_color();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cool_ppm() {
        let cols = 256;
        let rows = 256;
        let max_color = 255;

        let ppm = PPM::generate(
            cols, rows, max_color,
            |row, col| {

            let r = col / (cols-1) as Precision;
            let g = row / (rows-1) as Precision;
            let b = 0.;

            let r = r * 255.0;
            let g = g * 255.0;

            Color::new(r, g, b)
        });

        ppm.output();
    }
}
