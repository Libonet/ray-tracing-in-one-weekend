use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}};

pub type Precision = f32;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: Precision,
    y: Precision,
    z: Precision,
}

impl Vec3 {
    pub fn new(x: Precision, y: Precision, z: Precision) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> Precision {
        self.x
    }

    pub fn y(&self) -> Precision {
        self.y
    }

    pub fn z(&self) -> Precision {
        self.z
    }

    pub fn len(&self) -> Precision {
        Precision::sqrt(self.len_square())
    }

    pub fn len_square(&self) -> Precision {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)    
    }

    pub fn dot(&self, rhs: &Vec3) -> Precision {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.x * rhs.z - self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vec(self) -> Self {
        let len = self.len();
        self / len
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", &self.x, &self.y, &self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }    
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<i32> for Vec3 {
    type Output = Precision;
    
    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index!\n"),
        }
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Precision {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index!\n"),
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<Vec3> for Precision {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Precision> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Precision) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<Precision> for Vec3 {
    fn mul_assign(&mut self, rhs: Precision) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<Precision> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Precision) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<Precision> for Vec3 {
    fn div_assign(&mut self, rhs: Precision) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub type Point3 = Vec3;
