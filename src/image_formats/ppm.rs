
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PPM {
    cols: usize,
    rows: usize,
    max_color: u8,
    values: Vec<RGB>,
}

impl PPM {
    pub fn new(cols: usize, rows: usize, max_color: u8, values: Vec<RGB>) -> Self {
        assert!(values.len() == cols * rows);
        Self { cols, rows, max_color, values }
    }

    pub fn generate<F>(cols: usize, rows: usize, max_color: u8, gen: F) -> Self
    where 
        F: Fn(f64, f64) -> RGB,
    {
        let mut values = Vec::with_capacity(cols * rows);

        for row in 0..rows {
            for col in 0..cols {
                values.push(gen(row as f64, col as f64));
            }
        }

        PPM::new(cols, rows, max_color, values)
    }

    pub fn output(&self) {
        println!("P3");
        println!("{} {}", self.cols, self.rows);
        println!("{}", self.max_color);
        
        for row in 0..self.rows {
            eprintln!("Scanlines remaining: {}", self.rows - row);
            for col in 0..self.cols {
                let rgb = self.values[row * self.cols + col];
                println!("{} {} {}", rgb.r, rgb.g, rgb.b);
            }
        }

        eprintln!("Done! :D");
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

            let r = col / (cols-1) as f64;
            let g = row / (rows-1) as f64;
            let b = 0;

            let r = (r * 255.0) as u8;
            let g = (g * 255.0) as u8;

            RGB::new(r, g, b)
        });

        ppm.output();
    }
}
