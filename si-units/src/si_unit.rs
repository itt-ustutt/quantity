use crate::QuantityError;
use serde::{Deserialize, Serialize};
use std::ops::{Div, DivAssign, Mul, MulAssign, Rem};

/// Representation of a unit as a combination of SI base units.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct SIUnit(pub(crate) [i8; 7]);

impl SIUnit {
    /// Return the underlying unit vector.
    pub fn into_raw_parts(self) -> [i8; 7] {
        self.0
    }

    /// Build a unit from the underlying unit vector.
    pub fn from_raw_parts(unit: [i8; 7]) -> Self {
        Self(unit)
    }
}

impl SIUnit {
    pub const DIMENSIONLESS: Self = SIUnit([0; 7]);

    pub fn is_dimensionless(&self) -> bool {
        self == &Self::DIMENSIONLESS
    }

    pub const fn powi(&self, i: i32) -> Self {
        let i8 = i as i8;
        Self([
            self.0[0] * i8,
            self.0[1] * i8,
            self.0[2] * i8,
            self.0[3] * i8,
            self.0[4] * i8,
            self.0[5] * i8,
            self.0[6] * i8,
        ])
    }

    pub fn sqrt(&self) -> Result<Self, QuantityError> {
        self.root(2)
    }

    pub fn cbrt(&self) -> Result<Self, QuantityError> {
        self.root(3)
    }

    pub fn root(&self, i: i32) -> Result<Self, QuantityError> {
        let i8 = i as i8;
        if self.0.iter().all(|u| u.rem(i8) == 0) {
            Ok(Self([
                self.0[0] / i8,
                self.0[1] / i8,
                self.0[2] / i8,
                self.0[3] / i8,
                self.0[4] / i8,
                self.0[5] / i8,
                self.0[6] / i8,
            ]))
        } else {
            Err(QuantityError::InvalidRoot)
        }
    }

    pub fn recip(&self) -> Self {
        Self([
            -self.0[0], -self.0[1], -self.0[2], -self.0[3], -self.0[4], -self.0[5], -self.0[6],
        ])
    }
}

impl Mul for SIUnit {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
            self.0[4] + other.0[4],
            self.0[5] + other.0[5],
            self.0[6] + other.0[6],
        ])
    }
}

impl Div for SIUnit {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
            self.0[4] - other.0[4],
            self.0[5] - other.0[5],
            self.0[6] - other.0[6],
        ])
    }
}

impl MulAssign for SIUnit {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
        self.0[3] += rhs.0[3];
        self.0[4] += rhs.0[4];
        self.0[5] += rhs.0[5];
        self.0[6] += rhs.0[6];
    }
}

impl DivAssign for SIUnit {
    fn div_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
        self.0[3] -= rhs.0[3];
        self.0[4] -= rhs.0[4];
        self.0[5] -= rhs.0[5];
        self.0[6] -= rhs.0[6];
    }
}
