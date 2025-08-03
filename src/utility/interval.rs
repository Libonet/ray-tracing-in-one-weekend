use super::vec3::Precision;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: Precision,
    pub max: Precision,
}

impl Interval {
    pub fn new(min: Precision, max: Precision) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> Precision {
        self.max - self.min
    }

    pub fn contains(&self, x: Precision) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Precision) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: Precision) -> Precision {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }

    pub fn expand(&self, delta: Precision) -> Self {
        let padding = delta / 2.;
        Self { min: self.min - padding, max: self.max + padding }
    }

    pub const EMPTY: Self = Self { min: Precision::INFINITY, max: Precision::NEG_INFINITY };

    pub const UNIVERSE: Self = Self { min: Precision::NEG_INFINITY, max: Precision::INFINITY };
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
