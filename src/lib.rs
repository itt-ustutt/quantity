//! Representation of quantites, i.e. of unit valued scalars and arrays.
//!
//! As opposed to other implementations, this crate does not attempt to achieve compile time checks on units.
//! It is written with flexibility in mind and is able to represent arbitrarily complex units.
//! Additional to simple scalar quantities, it also provides utilities for vector valued quantities, based on the [ndarray] crate, where all entries share the same unit.
//!
//! For details on the available methods, see the documentation of the [Quantity] struct and for a list of all units and constants available check out the [si] module.
//!
//! ## Examples
//!
//! Calculate pressure of an ideal gas.
//! ```
//! # use quantity::si::*;
//! let temperature = 25.0 * CELSIUS;
//! let volume = 1.5 * METER.powi(3);
//! let moles = 75.0 * MOL;
//! let pressure = moles * RGAS * temperature / volume;
//! println!("{:.5}", pressure);            // 123.94785 kPa
//! ```
//!
//! Calculate the gravitational pull of the moon on the earth.
//! ```
//! # use quantity::si::*;
//! let mass_earth = 5.9724e24 * KILOGRAM;
//! let mass_moon = 7.346e22 * KILOGRAM;
//! let distance = 383.398 * KILO * METER;
//! let force = G * mass_earth * mass_moon / distance.powi(2);
//! println!("{:.5e}", force);              // 1.99208e26 N
//! ```
//!
//! Calculate the pressure distribution in the atmosphere using the barometric formula.
//! ```
//! # use quantity::si::*;
//! # use quantity::QuantityError;
//! # fn main() -> Result<(), QuantityError> {
//! let z = SIArray1::linspace(1.0 * METER, 70.0 * KILO * METER, 10)?;
//! let g = 9.81 * METER / SECOND.powi(2);
//! let m = 28.949 * GRAM / MOL;
//! let t = 10.0 * CELSIUS;
//! let p0 = BAR;
//! let pressure = p0 * (-&z * m * g).to_reduced(RGAS * t)?.mapv(f64::exp);
//! for i in 0..10 {
//!     println!("z = {:8.5}   p = {:9.5}", z.get(i), pressure.get(i));
//! }
//! // z =  1.00000  m   p =  99.98794 kPa
//! // z =  7.77867 km   p =  39.12796 kPa
//! // z = 15.55633 km   p =  15.31182 kPa
//! // z = 23.33400 km   p =   5.99192 kPa
//! // z = 31.11167 km   p =   2.34480 kPa
//! // z = 38.88933 km   p = 917.58301  Pa
//! // z = 46.66700 km   p = 359.07479  Pa
//! // z = 54.44467 km   p = 140.51557  Pa
//! // z = 62.22233 km   p =  54.98750  Pa
//! // z = 70.00000 km   p =  21.51808  Pa
//! # Ok(())
//! # }
//! ```

#![warn(clippy::all)]
use approx::{AbsDiffEq, RelativeEq};
use ndarray::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use thiserror::Error;

#[cfg(feature = "linalg")]
mod linalg;
#[cfg(feature = "python")]
pub mod python;
pub mod si;
mod si_fmt;

/// Error type used to indicate unit conversion failures.
#[derive(Error, Debug)]
pub enum QuantityError {
    #[error(
        "Operation '{op}' encountered incompatible units. Expected {expected} but found {found}."
    )]
    UnitError {
        op: String,
        expected: String,
        found: String,
    },
    #[error("Operation '{op}' failed. {cause}")]
    SINumberError { op: String, cause: String },
}

/// A generalized unit.
pub trait Unit:
    Copy + PartialEq + Div<Output = Self> + Mul<Output = Self> + DivAssign + MulAssign + fmt::Display
{
    /// The value of Self that corresponds to a dimensionless property.
    const DIMENSIONLESS: Self;

    /// Returns `true` if `self` is dimensionless and `false` otherwise.
    fn is_dimensionless(&self) -> bool {
        self.eq(&Self::DIMENSIONLESS)
    }

    /// Calculate the integer power of self.
    fn powi(&self, i: i32) -> Self;
    /// Try to calculate the square root of self.
    fn sqrt(&self) -> Result<Self, QuantityError>;
    /// Try to calculate the cubic root of self.
    fn cbrt(&self) -> Result<Self, QuantityError>;
    /// Try to calculate the integer root of self.
    fn root(&self, i: i32) -> Result<Self, QuantityError>;
}

/// Representation of a value with a corresponding unit.
///
/// ## Contents
///
/// + [Methods for All Quantities](#methods-for-all-quantities)
/// + [Methods for Scalar Quantities](#methods-for-scalar-quantities)
/// + [Methods for n-Dimensional Array Quantities](#methods-for-n-dimensional-array-quantities)
/// + [Methods for 1-Dimensional Array Quantities](#methods-for-1-dimensional-array-quantities)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<F, U> {
    pub(crate) value: F,
    pub(crate) unit: U,
}

pub type QuantityScalar<U> = Quantity<f64, U>;
pub type QuantityArray<U, D> = Quantity<Array<f64, D>, U>;
pub type QuantityArray0<U> = QuantityArray<U, Ix0>;
pub type QuantityArray1<U> = QuantityArray<U, Ix1>;
pub type QuantityArray2<U> = QuantityArray<U, Ix2>;
pub type QuantityArray3<U> = QuantityArray<U, Ix3>;
pub type QuantityArray4<U> = QuantityArray<U, Ix4>;
pub type QuantityArray5<U> = QuantityArray<U, Ix5>;
pub type QuantityArray6<U> = QuantityArray<U, Ix6>;

/// # Methods for All Quantities
impl<F, U: Unit> Quantity<F, U> {
    /// Check if the quantity has the same unit as the argument.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::*;
    /// # use quantity::QuantityError;
    /// let p = 5.0 * NEWTON/METER.powi(2);
    /// assert!(p.has_unit(&BAR));
    /// ```
    pub fn has_unit<F2>(&self, other: &Quantity<F2, U>) -> bool {
        self.unit.eq(&other.unit)
    }

    /// Return a reference to its value if the quantity is dimensionless.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::PASCAL;
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let p1 = 5.0 * PASCAL;
    /// let p2 = 2.5 * PASCAL;
    /// let ratio = p1 / p2;
    /// assert_relative_eq!(ratio.value()?, &2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn value(&self) -> Result<&F, QuantityError> {
        if self.unit.is_dimensionless() {
            Ok(&self.value)
        } else {
            Err(QuantityError::UnitError {
                op: "value".to_string(),
                expected: U::DIMENSIONLESS.to_string(),
                found: self.unit.to_string(),
            })
        }
    }

    /// Converts to its value if the quantity is dimensionless.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::PASCAL;
    /// # use quantity::QuantityError;
    /// # use approx::{assert_relative_eq, assert_ulps_eq};
    /// # fn main() -> Result<(), QuantityError> {
    /// let p1 = 5.0 * PASCAL;
    /// let p2 = 2.5 * PASCAL;
    /// let ratio = p1 / p2;
    /// assert_relative_eq!(ratio.into_value()?, 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_value(self) -> Result<F, QuantityError> {
        if self.unit.is_dimensionless() {
            Ok(self.value)
        } else {
            Err(QuantityError::UnitError {
                op: "into_value".to_string(),
                expected: U::DIMENSIONLESS.to_string(),
                found: self.unit.to_string(),
            })
        }
    }

    /// Returns the value of self in a given unit if possible.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::{BAR, PASCAL};
    /// # use quantity::QuantityError;
    /// # use approx::{assert_relative_eq, assert_ulps_eq};
    /// # fn main() -> Result<(), QuantityError> {
    /// let p = 5.0 * BAR;
    /// assert_relative_eq!(p.to_reduced(PASCAL)?, 500000.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_reduced<'a>(&'a self, reference: QuantityScalar<U>) -> Result<F, QuantityError>
    where
        &'a Self: Div<QuantityScalar<U>, Output = Self>,
    {
        (self / reference).into_value()
    }
}

impl<F, U: Unit> From<F> for Quantity<F, U> {
    fn from(value: F) -> Self {
        Self {
            value,
            unit: U::DIMENSIONLESS,
        }
    }
}

impl<F: Neg, U> Neg for Quantity<F, U> {
    type Output = Quantity<<F as Neg>::Output, U>;
    fn neg(self) -> Self::Output {
        Quantity {
            value: -self.value,
            unit: self.unit,
        }
    }
}

impl<'a, F, U> Neg for &'a Quantity<F, U>
where
    &'a F: Neg,
    U: Copy,
{
    type Output = Quantity<<&'a F as Neg>::Output, U>;
    fn neg(self) -> Self::Output {
        Quantity {
            value: -&self.value,
            unit: self.unit,
        }
    }
}

macro_rules! impl_mul_op {
    ($trt:ident, $operator:tt, $mth:ident, $trt_assign:ident, $op_assign:tt, $mth_assign:ident, $exp:literal) => {
        impl<F, F2, U> $trt<Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt<F2>,
            U: $trt<Output = U>,
        {
            type Output = Quantity<<F as $trt<F2>>::Output, U>;
            fn $mth(self, other: Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: self.value $operator other.value,
                    unit: self.unit $operator other.unit,
                }
            }
        }

        impl<'a, F, F2, U> $trt<Quantity<F2, U>> for &'a Quantity<F, U>
        where
            &'a F: $trt<F2>,
            U: $trt<Output = U> + Copy,
        {
            type Output = Quantity<<&'a F as $trt<F2>>::Output, U>;
            fn $mth(self, other: Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: &self.value $operator other.value,
                    unit: self.unit $operator other.unit,
                }
            }
        }

        impl<'a, F, F2, U> $trt<&'a Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt<&'a F2>,
            U: $trt<Output = U> + Copy,
        {
            type Output = Quantity<<F as $trt<&'a F2>>::Output, U>;
            fn $mth(self, other: &'a Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: self.value $operator &other.value,
                    unit: self.unit $operator other.unit,
                }
            }
        }

        impl<'a, 'b, F, F2, U> $trt<&'b Quantity<F2, U>> for &'a Quantity<F, U>
        where
            &'a F: $trt<&'b F2>,
            U: $trt<Output = U> + Copy,
        {
            type Output = Quantity<<&'a F as $trt<&'b F2>>::Output, U>;
            fn $mth(self, other: &'b Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: &self.value $operator &other.value,
                    unit: self.unit $operator other.unit,
                }
            }
        }

        impl<F, U> $trt<f64> for Quantity<F, U>
        where
            F: $trt<f64>,
        {
            type Output = Quantity<<F as $trt<f64>>::Output, U>;
            fn $mth(self, other: f64) -> Self::Output {
                Quantity {
                    value: self.value $operator other,
                    unit: self.unit,
                }
            }
        }

        impl<'a, F, U> $trt<f64> for &'a Quantity<F, U>
        where
            &'a F: $trt<f64>,
            U: Copy,
        {
            type Output = Quantity<<&'a F as $trt<f64>>::Output, U>;
            fn $mth(self, other: f64) -> Self::Output {
                Quantity {
                    value: &self.value $operator other,
                    unit: self.unit,
                }
            }
        }

        impl<F, U> $trt<Quantity<F, U>> for f64
        where
            f64: $trt<F>,
            U: Unit,
        {
            type Output = Quantity<<f64 as $trt<F>>::Output, U>;
            fn $mth(self, other: Quantity<F, U>) -> Self::Output {
                Quantity {
                    value: self $operator other.value,
                    unit: other.unit.powi($exp),
                }
            }
        }

        impl<F, U, S: RawData, D> $trt<ArrayBase<S, D>> for Quantity<F, U>
        where
            F: $trt<ArrayBase<S, D>>,
        {
            type Output = Quantity<<F as $trt<ArrayBase<S, D>>>::Output, U>;
            fn $mth(self, other: ArrayBase<S, D>) -> Self::Output {
                Quantity {
                    value: self.value $operator other,
                    unit: self.unit,
                }
            }
        }

        impl<'a, F, U, S: RawData, D> $trt<&'a ArrayBase<S, D>> for Quantity<F, U>
        where
            F: $trt<&'a ArrayBase<S, D>>,
        {
            type Output = Quantity<<F as $trt<&'a ArrayBase<S, D>>>::Output, U>;
            fn $mth(self, other: &'a ArrayBase<S, D>) -> Self::Output {
                Quantity {
                    value: self.value $operator other,
                    unit: self.unit,
                }
            }
        }

        impl<F, U, S: RawData, D> $trt<Quantity<F, U>> for ArrayBase<S, D>
        where
            ArrayBase<S, D>: $trt<F>,
            U: Unit,
        {
            type Output = Quantity<<ArrayBase<S, D> as $trt<F>>::Output, U>;
            fn $mth(self, other: Quantity<F, U>) -> Self::Output {
                Quantity {
                    value: self $operator other.value,
                    unit: other.unit.powi($exp),
                }
            }
        }

        impl<'a, F, U, S: RawData, D> $trt<Quantity<F, U>> for &'a ArrayBase<S, D>
        where
            &'a ArrayBase<S, D>: $trt<F>,
            U: Unit,
        {
            type Output = Quantity<<&'a ArrayBase<S, D> as $trt<F>>::Output, U>;
            fn $mth(self, other: Quantity<F, U>) -> Self::Output {
                Quantity {
                    value: self $operator other.value,
                    unit: other.unit,
                }
            }
        }

        impl<F, F2, U> $trt_assign<Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt_assign<F2>,
            U: $trt_assign,
        {
            fn $mth_assign(&mut self, other: Quantity<F2, U>) {
                self.value $op_assign other.value;
                self.unit $op_assign other.unit;
            }
        }

        impl<'a, F, F2, U> $trt_assign<&'a Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt_assign<&'a F2>,
            U: $trt_assign + Copy,
        {
            fn $mth_assign(&mut self, other: &'a Quantity<F2, U>) {
                self.value $op_assign &other.value;
                self.unit $op_assign other.unit;
            }
        }
    };
}

impl_mul_op!(Mul, *, mul, MulAssign, *=, mul_assign, 1);
impl_mul_op!(Div, /, div, DivAssign, /=, div_assign, -1);

macro_rules! impl_add_op {
    ($trt:ident, $operator:tt, $mth:ident, $trt_assign:ident, $op_assign:tt, $mth_assign:ident) => {
        impl<F, F2, U> $trt<Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt<F2>,
            U: PartialEq + fmt::Display,
        {
            type Output = Quantity<<F as $trt<F2>>::Output, U>;
            fn $mth(self, other: Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: self.value $operator other.value,
                    unit: if self.unit == other.unit {
                        self.unit
                    } else {
                        panic!("Inconsistent units {} {} {}", self.unit, stringify!($operator), other.unit)
                    },
                }
            }
        }

        impl<'a, F, F2, U> $trt<Quantity<F2, U>> for &'a Quantity<F, U>
        where
            &'a F: $trt<F2>,
            U: Copy + PartialEq + fmt::Display,
        {
            type Output = Quantity<<&'a F as $trt<F2>>::Output, U>;
            fn $mth(self, other: Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: &self.value $operator other.value,
                    unit: if self.unit == other.unit {
                        self.unit
                    } else {
                        panic!("Inconsistent units {} {} {}", self.unit, stringify!($operator), other.unit)
                    },
                }
            }
        }

        impl<'a, F, F2, U> $trt<&'a Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt<&'a F2>,
            U: PartialEq + fmt::Display,
        {
            type Output = Quantity<<F as $trt<&'a F2>>::Output, U>;
            fn $mth(self, other: &'a Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: self.value $operator &other.value,
                    unit: if self.unit == other.unit {
                        self.unit
                    } else {
                        panic!("Inconsistent units {} {} {}", self.unit, stringify!($operator), other.unit)
                    },
                }
            }
        }

        impl<'a, 'b, F, F2, U> $trt<&'b Quantity<F2, U>> for &'a Quantity<F, U>
        where
            &'a F: $trt<&'b F2>,
            U: Copy + PartialEq + fmt::Display,
        {
            type Output = Quantity<<&'a F as $trt<&'b F2>>::Output, U>;
            fn $mth(self, other: &'b Quantity<F2, U>) -> Self::Output {
                Quantity {
                    value: &self.value $operator &other.value,
                    unit: if self.unit == other.unit {
                        self.unit
                    } else {
                        panic!("Inconsistent units {} {} {}", self.unit, stringify!($operator), other.unit)
                    },
                }
            }
        }

        impl<F, F2, U> $trt_assign<Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt_assign<F2>,
            U: PartialEq + fmt::Display,
        {
            fn $mth_assign(&mut self, other: Quantity<F2, U>) {
                if self.unit != other.unit {
                    panic!("Inconsistent units {} {} {}", self.unit, stringify!($op_assign), other.unit)
                }
                self.value $op_assign other.value;
            }
        }

        impl<'a, F, F2, U> $trt_assign<&'a Quantity<F2, U>> for Quantity<F, U>
        where
            F: $trt_assign<&'a F2>,
            U: PartialEq + fmt::Display,
        {
            fn $mth_assign(&mut self, other: &'a Quantity<F2, U>) {
                if self.unit != other.unit {
                    panic!("Inconsistent units {} {} {}", self.unit, stringify!($op_assign), other.unit)
                }
                self.value $op_assign &other.value;
            }
        }
    };
}

impl_add_op!(Add, +, add, AddAssign, += , add_assign);
impl_add_op!(Sub, -, sub, SubAssign, -= , sub_assign);

impl<F: PartialEq, U: PartialEq + fmt::Display> PartialEq for Quantity<F, U> {
    fn eq(&self, other: &Self) -> bool {
        if self.unit.eq(&other.unit) {
            self.value.eq(&other.value)
        } else {
            panic!("Inconsistent units {} and {}", self.unit, other.unit)
        }
    }
}

impl<F: PartialOrd, U: PartialEq + fmt::Display> PartialOrd for Quantity<F, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit.eq(&other.unit) {
            self.value.partial_cmp(&other.value)
        } else {
            panic!("Inconsistent units {} and {}", self.unit, other.unit)
        }
    }
}

impl<F: AbsDiffEq, U: PartialEq + fmt::Display> AbsDiffEq for Quantity<F, U> {
    type Epsilon = <F as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        <F as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.unit.eq(&other.unit) {
            self.value.abs_diff_eq(&other.value, epsilon)
        } else {
            false
        }
    }
}

impl<F: RelativeEq, U: PartialEq + fmt::Display> RelativeEq for Quantity<F, U> {
    fn default_max_relative() -> Self::Epsilon {
        <F as RelativeEq>::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        if self.unit.eq(&other.unit) {
            self.value.relative_eq(&other.value, epsilon, max_relative)
        } else {
            false
        }
    }
}

/// # Methods for Scalar Quantities
impl<U: Unit> QuantityScalar<U> {
    /// Calculate the integer power of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use approx::assert_relative_eq;
    /// let x = 5.0 * METER;
    /// assert_relative_eq!(x.powi(2), &(25.0 * METER * METER));
    /// ```
    pub fn powi(&self, i: i32) -> Self {
        Quantity {
            value: self.value.powi(i),
            unit: self.unit.powi(i),
        }
    }

    /// Try to calculate the square root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = 25.0 * METER * METER;
    /// assert_relative_eq!(x.sqrt()?, &(5.0 * METER));
    /// assert!(METER.sqrt().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn sqrt(&self) -> Result<Self, QuantityError> {
        Ok(Quantity {
            value: self.value.sqrt(),
            unit: self.unit.sqrt()?,
        })
    }

    /// Try to calculate the cubic root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = 125.0 * METER * METER * METER;
    /// assert_relative_eq!(x.cbrt()?, &(5.0 * METER));
    /// assert!(METER.cbrt().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cbrt(&self) -> Result<Self, QuantityError> {
        Ok(Quantity {
            value: self.value.cbrt(),
            unit: self.unit.cbrt()?,
        })
    }

    /// Try to calculate the integer root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = 625.0 * METER * METER * METER * METER;
    /// assert_relative_eq!(x.root(4)?, &(5.0 * METER));
    /// assert!(METER.root(4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn root(&self, i: i32) -> Result<Self, QuantityError> {
        Ok(Quantity {
            value: self.value.powf(1.0 / i as f64),
            unit: self.unit.root(i)?,
        })
    }

    /// Return the maximum of `self` and `other` if they have the same unit.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::{KILO, PASCAL, BAR, KELVIN};
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let p1 = 110.0 * KILO * PASCAL;
    /// let p2 = BAR;
    /// assert_relative_eq!(p1.max(p2)?, &p1);
    /// assert!(BAR.max(KELVIN).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn max(&self, other: Self) -> Result<Self, QuantityError>
    where
        U: PartialEq + Clone,
    {
        if self.unit == other.unit {
            Ok(Self {
                value: self.value.max(other.value),
                unit: self.unit,
            })
        } else {
            Err(QuantityError::UnitError {
                op: "max".to_owned(),
                expected: self.unit.to_string(),
                found: other.unit.to_string(),
            })
        }
    }

    /// Return the minimum of `self` and `other` if they have the same unit.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::{KILO, PASCAL, BAR, KELVIN};
    /// # use quantity::QuantityError;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let p1 = 110.0 * KILO * PASCAL;
    /// let p2 = BAR;
    /// assert_relative_eq!(p1.min(p2)?, &p2);
    /// assert!(BAR.min(KELVIN).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn min(&self, other: Self) -> Result<Self, QuantityError>
    where
        U: PartialEq + Clone,
    {
        if self.unit == other.unit {
            Ok(Self {
                value: self.value.min(other.value),
                unit: self.unit,
            })
        } else {
            Err(QuantityError::UnitError {
                op: "min".to_owned(),
                expected: self.unit.to_string(),
                found: other.unit.to_string(),
            })
        }
    }

    /// Return the absolute value of `self`.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::KELVIN;
    /// # use approx::assert_relative_eq;
    /// let t = -50.0 * KELVIN;
    /// assert_relative_eq!(t.abs(), &(50.0 * KELVIN));
    /// ```
    pub fn abs(&self) -> Self
    where
        U: Clone,
    {
        Self {
            value: self.value.abs(),
            unit: self.unit,
        }
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    pub fn signum(&self) -> f64 {
        self.value.signum()
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`, `NaN`s with
    /// positive sign bit and positive infinity.
    pub fn is_sign_positive(&self) -> bool {
        self.value.is_sign_positive()
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`, `NaN`s with
    /// negative sign bit and negative infinity.
    pub fn is_sign_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    /// Returns true if this value is NaN.
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }
}

/// # Methods for n-Dimensional Array Quantities
impl<U: Unit, D: Dimension, S: Data<Elem = f64>> Quantity<ArrayBase<S, D>, U> {
    /// Return the sum of all elements in the array.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::BAR;
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// let x = arr1(&[1.5, 2.5]) * BAR;
    /// assert_relative_eq!(x.sum(), &(4.0 * BAR));
    /// ```
    pub fn sum(&self) -> QuantityScalar<U> {
        QuantityScalar {
            value: self.value.sum(),
            unit: self.unit,
        }
    }

    /// Return the total number of elements in the array.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Return whether the array has any elements
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Return the shape of the array as it’s stored in the array.
    pub fn raw_dim(&self) -> D {
        self.value.raw_dim()
    }

    /// Return the shape of the array as a slice.
    pub fn shape(&self) -> &[usize] {
        self.value.shape()
    }

    /// Return an uniquely owned copy of the array.
    pub fn to_owned(&self) -> QuantityArray<U, D> {
        Quantity {
            value: self.value.to_owned(),
            unit: self.unit,
        }
    }

    /// Return the element at `index`.
    ///
    /// The `Index` trait can not be implemented, because a new instance has to be created,
    /// when indexing a quantity array. This serves as replacement for it.
    pub fn get<I: NdIndex<D>>(&self, index: I) -> QuantityScalar<U> {
        QuantityScalar {
            value: *self.value.index(index),
            unit: self.unit,
        }
    }

    /// Set the element at `index` to `scalar` if scalar has the same unit as `self`.
    pub fn try_set<I: NdIndex<D>>(
        &mut self,
        index: I,
        scalar: QuantityScalar<U>,
    ) -> Result<(), QuantityError>
    where
        S: DataMut,
    {
        if self.has_unit(&scalar) {
            *self.value.index_mut(index) = scalar.value;
            Ok(())
        } else {
            Err(QuantityError::UnitError {
                op: "try_set".to_owned(),
                expected: self.unit.to_string(),
                found: scalar.unit.to_string(),
            })
        }
    }

    /// Returns a view restricted to index along the axis, with the axis removed.
    pub fn index_axis(
        &self,
        axis: Axis,
        index: usize,
    ) -> Quantity<ArrayView<'_, f64, D::Smaller>, U>
    where
        D: RemoveAxis,
    {
        Quantity {
            value: self.value.index_axis(axis, index),
            unit: self.unit,
        }
    }

    /// Insert new array axis at axis and return the result.
    pub fn insert_axis(self, axis: Axis) -> Quantity<ArrayBase<S, D::Larger>, U> {
        Quantity {
            value: self.value.insert_axis(axis),
            unit: self.unit,
        }
    }

    /// Return sum along axis.
    pub fn sum_axis(&self, axis: Axis) -> Quantity<Array<f64, D::Smaller>, U>
    where
        D: RemoveAxis,
    {
        Quantity {
            value: self.value.sum_axis(axis),
            unit: self.unit,
        }
    }

    /// Return a vector of scalar quantities for each element of `self`.
    pub fn to_vec(&self) -> Vec<QuantityScalar<U>> {
        self.value
            .iter()
            .map(|v| QuantityScalar {
                value: *v,
                unit: self.unit,
            })
            .collect()
    }

    /// Create an array with values created by the function f.
    pub fn from_shape_fn<Sh, F>(shape: Sh, mut f: F) -> QuantityArray<U, D>
    where
        Sh: ShapeBuilder<Dim = D>,
        F: FnMut(D::Pattern) -> QuantityScalar<U>,
    {
        let mut unit = U::DIMENSIONLESS;
        let value = Array::from_shape_fn(shape, |x| {
            let q = f(x);
            if unit != U::DIMENSIONLESS && unit != q.unit {
                panic!("Inconsistent units {} and {}", unit, q.unit);
            } else {
                unit = q.unit;
            }
            q.value
        });
        QuantityArray { value, unit }
    }

    /// Calculate the integer power of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// let x = arr1(&[3.0, 5.0]) * METER;
    /// assert_relative_eq!(x.powi(2), &(arr1(&[9.0, 25.0]) * METER * METER));
    /// ```
    pub fn powi(&self, i: i32) -> QuantityArray<U, D> {
        Quantity {
            value: self.value.mapv(|x| x.powi(i)),
            unit: self.unit.powi(i),
        }
    }

    /// Try to calculate the square root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = arr1(&[9.0, 25.0]) * METER * METER;
    /// assert_relative_eq!(x.sqrt()?, &(arr1(&[3.0, 5.0]) * METER));
    /// # Ok(())
    /// # }
    /// ```
    pub fn sqrt(&self) -> Result<QuantityArray<U, D>, QuantityError> {
        Ok(Quantity {
            value: self.value.mapv(|x| x.sqrt()),
            unit: self.unit.sqrt()?,
        })
    }

    /// Try to calculate the cubic root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = arr1(&[27.0, 125.0]) * METER * METER * METER;
    /// assert_relative_eq!(x.cbrt()?, &(arr1(&[3.0, 5.0]) * METER));
    /// # Ok(())
    /// # }
    /// ```
    pub fn cbrt(&self) -> Result<QuantityArray<U, D>, QuantityError> {
        Ok(Quantity {
            value: self.value.mapv(|x| x.cbrt()),
            unit: self.unit.cbrt()?,
        })
    }

    /// Try to calculate the integer root of self.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::METER;
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = arr1(&[81.0, 625.0]) * METER * METER * METER * METER;
    /// assert_relative_eq!(x.root(4)?, &(arr1(&[3.0, 5.0]) * METER));
    /// # Ok(())
    /// # }
    /// ```
    pub fn root(&self, i: i32) -> Result<QuantityArray<U, D>, QuantityError> {
        Ok(Quantity {
            value: self.value.mapv(|x| x.powf(1.0 / i as f64)),
            unit: self.unit.root(i)?,
        })
    }
}

/// # Methods for 1-Dimensional Array Quantities
impl<U: Unit> QuantityArray1<U> {
    /// Create a one-dimensional array with n evenly spaced elements from `start` to `end` (inclusive) if `start` and `end` have the same unit.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::{SIArray1, METER};
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = SIArray1::linspace(1.0 * METER, 3.0 * METER, 5)?;
    /// assert_relative_eq!(x, &(arr1(&[1.0, 1.5, 2.0, 2.5, 3.0]) * METER));
    /// # Ok(())
    /// # }
    /// ```
    pub fn linspace(
        start: QuantityScalar<U>,
        end: QuantityScalar<U>,
        n: usize,
    ) -> Result<Self, QuantityError> {
        if start.has_unit(&end) {
            Ok(Quantity {
                value: Array1::linspace(start.value, end.value, n),
                unit: start.unit,
            })
        } else {
            Err(QuantityError::UnitError {
                op: "linspace".to_owned(),
                expected: start.unit.to_string(),
                found: end.unit.to_string(),
            })
        }
    }

    /// Create a one-dimensional array with n logarithmically spaced elements from `start` to `end` (inclusive) if `start` and `end` have the same unit.
    ///
    /// # Example
    /// ```
    /// # use quantity::si::{SIArray1, METER};
    /// # use quantity::QuantityError;
    /// # use ndarray::arr1;
    /// # use approx::assert_relative_eq;
    /// # fn main() -> Result<(), QuantityError> {
    /// let x = SIArray1::logspace(1.0 * METER, 16.0 * METER, 5)?;
    /// assert_relative_eq!(x, &(arr1(&[1.0, 2.0, 4.0, 8.0, 16.0]) * METER));
    /// # Ok(())
    /// # }
    /// ```
    pub fn logspace(
        start: QuantityScalar<U>,
        end: QuantityScalar<U>,
        n: usize,
    ) -> Result<Self, QuantityError> {
        if start.has_unit(&end) {
            Ok(Quantity {
                value: Array1::logspace(10.0, start.value.log10(), end.value.log10(), n),
                unit: start.unit,
            })
        } else {
            Err(QuantityError::UnitError {
                op: "logspace".to_owned(),
                expected: start.unit.to_string(),
                found: end.unit.to_string(),
            })
        }
    }

    /// Create a one-dimensional array from a vector of scalar quantities.
    pub fn from_vec(vec: Vec<QuantityScalar<U>>) -> Self {
        Self::from_shape_fn(vec.len(), |i| vec[i])
    }
}

pub struct QuantityIter<I, U> {
    inner: I,
    unit: U,
}

impl<'a, I: Iterator<Item = &'a f64>, U: Copy> Iterator for QuantityIter<I, U> {
    type Item = QuantityScalar<U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|value| QuantityScalar {
            value: *value,
            unit: self.unit,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, I: Iterator<Item = &'a f64> + ExactSizeIterator, U: Copy> ExactSizeIterator
    for QuantityIter<I, U>
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, I: Iterator<Item = &'a f64> + DoubleEndedIterator, U: Copy> DoubleEndedIterator
    for QuantityIter<I, U>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|value| QuantityScalar {
            value: *value,
            unit: self.unit,
        })
    }
}

impl<'a, F, U: Copy> IntoIterator for &'a Quantity<F, U>
where
    &'a F: IntoIterator<Item = &'a f64>,
{
    type Item = QuantityScalar<U>;
    type IntoIter = QuantityIter<<&'a F as IntoIterator>::IntoIter, U>;

    fn into_iter(self) -> Self::IntoIter {
        QuantityIter {
            inner: (&self.value).into_iter(),
            unit: self.unit,
        }
    }
}

impl<U: Unit> FromIterator<QuantityScalar<U>> for QuantityArray1<U> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = QuantityScalar<U>>,
    {
        let (value, unit) =
            iter.into_iter()
                .fold((Vec::new(), U::DIMENSIONLESS), |(mut value, unit), q| {
                    value.push(q.value);
                    if unit != q.unit && unit != U::DIMENSIONLESS {
                        panic!("Inconsistent units {unit} and {}", q.unit);
                    }
                    (value, q.unit)
                });
        let value = Array1::from_vec(value);
        Self { value, unit }
    }
}

#[cfg(test)]
mod test {
    use crate::si::*;
    use std::iter::once;

    #[test]
    fn test_into_iter() {
        let x = SIArray1::linspace(1.0 * METER, 3.0 * METER, 6).unwrap();
        for q in &x {
            println!("{q}");
        }
        let mut x_iter = x.into_iter();
        assert_eq!(x_iter.next(), Some(1.0 * METER));
        assert_eq!(x_iter.next(), Some(1.4 * METER));
        assert_eq!(x_iter.next(), Some(1.8 * METER));
        assert_eq!(x_iter.next(), Some(2.2 * METER));
        assert_eq!(x_iter.next(), Some(2.6 * METER));
        assert_eq!(x_iter.next(), Some(3.0 * METER));
        assert_eq!(x_iter.next(), None);
    }

    #[test]
    fn test_collect_vec() {
        let vec: Vec<_> = once(KELVIN).chain(once(METER)).collect();
        assert_eq!(vec[0], KELVIN);
        assert_eq!(vec[1], METER);
    }

    #[test]
    #[should_panic(expected = "Inconsistent units K and m")]
    fn test_collect_array_wrong() {
        let arr: SIArray1 = once(KELVIN).chain(once(METER)).collect();
        println!("{arr}");
    }

    #[test]
    fn test_collect_array_correct() {
        let arr: SIArray1 = once(KELVIN).chain(once(25.0 * CELSIUS)).collect();
        println!("{arr}");
        assert_eq!(arr.get(0), KELVIN);
        assert_eq!(arr.get(1), 25.0 * CELSIUS);
    }
}
