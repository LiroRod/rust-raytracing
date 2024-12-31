use std::fmt;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

// pub here makes the vec3 struct public, meaning it can be accessed outside of the module or crate in which it is defined.
// Structs in Rust are used to group related data todether.

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3], // The struct has a single field named "e" and its type is [f64; 3] which means
                 // that it is an array of 3 elements, each of type f64 (64-bit floating point)
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    } // Takes three values and returns a new Vec3 instance with these values as its components;

    pub fn zero() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    } // Returns a Vec3 instance representing the zero vector: [0.0, 0.0, 0.0];

    pub fn x(&self) -> f64 {
        self.e[0]
    } // Accessor for the x-compontent of the vector self.e[0]
    pub fn y(&self) -> f64 {
        self.e[1]
    } // Accessor for the x-compontent of the vector self.e[0]
    pub fn z(&self) -> f64 {
        self.e[2]
    } // Accessor for the x-compontent of the vector self.e[0]

    pub fn length(&self) -> f64 {
        // computes the length (magnitude) of the vector using the
        // formula "len = sqrt(x^2 + y^2 + z^2 )"
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        // Computes the squared length of the vector. Useful for
        // performance when the exact length isn't needed (avoids
        // the square root calculation)
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        // Computes the dot product with another vector. ie "dot = X1X2 + Y1Y2 + Z1Z2"
        // returns a scalar (f64)
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        // computes the cross product with another vector, resulting in a new vector perpendicular
        // to both, returns a Vec3 insntace.
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

// Allows Vec3 instances to be used with {} in formatted strings (e.g, ```println!("{}", vec)```)
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
// usage:
// let vec = Vec3::new(1.0, 2.0, 3.0)
// println!("{}", vec); >>> outputs: "1.0, 2.0, 3.0"

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[1]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<Vec3> for Vec3 {
    // This is an element-wise multiplication between two vectors v1 * v2
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    // This is an scalar multiplication for Vec3 vec * scalar
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    // This is an inverse scalar multiplication for Vec3 scalar * vec
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_basic_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1 + v2, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(v1 + 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(v1.dot(&v2), 20.0);
    }
}
