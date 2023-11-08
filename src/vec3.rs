use std::ops;

use crate::utility::{random, random_in_range};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0; 3] }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        //Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        f64::abs(self.e[0]) < s && f64::abs(self.e[1]) < s && f64::abs(self.e[2]) < s
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Self {
        Self::from(random(), random(), random())
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self::from(
            random_in_range(min, max),
            random_in_range(min, max),
            random_in_range(min, max),
        )
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.e
        .iter()
        .zip(rhs.e.iter())
        .fold(0.0, |acc, p| acc + p.0 * p.1)
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::from(
        lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
        lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
        lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();

    if dot(normal, &on_unit_sphere) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::from(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::from(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::Vec3;

    #[test]
    fn basic() {
        // Create an empty point as origin
        // And check all values are 0.0
        let origin = Vec3::new();
        assert_eq!(origin.x(), 0.0);
        assert_eq!(origin.y(), 0.0);
        assert_eq!(origin.z(), 0.0);

        // Create a direction of vector
        // And check the value
        let direction = Vec3::from(1.0, 2.0, 3.0);
        assert_eq!(direction.x(), 1.0);
        assert_eq!(direction.y(), 2.0);
        assert_eq!(direction.z(), 3.0);

        // Test operator [+]
        let point = origin + direction;
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);

        // Test operator [*]
        let point = point * 5.0;
        assert_eq!(point.x(), 5.0);
        assert_eq!(point.y(), 10.0);
        assert_eq!(point.z(), 15.0);

        // Test operator [/]
        let mut point = point / 5.0;
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);

        // Tset operator [*=] and [/=]
        point *= 5.0;
        point /= 5.0;
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);

        // Test indexer operator
        assert_eq!(point[0], 1.0);
        assert_eq!(point[1], 2.0);
        assert_eq!(point[2], 3.0);

        // Test mutable indexer
        let mut point = point;
        point[0] = 0.0;
        point[1] = 3.0;
        point[2] = 4.0;

        assert_eq!(point.length_squared(), 25.0);
        approx::assert_relative_eq!(point.length(), f64::sqrt(25.0))
    }

    #[test]
    fn vec_operation() {
        let u = Vec3::from(1.0, 0.0, 0.0);
        let v = Vec3::from(10.0, 0.0, 0.0);

        assert_eq!(unit_vector(v), u);
        assert_eq!(dot(&u, &v), 10.0);
        assert_eq!(cross(&u, &v), Vec3::new())
    }
}
