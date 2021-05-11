//! Implementation of SI units.
//!
//! ## SI Base Units and Associated Constants
//!
//! The module contains the seven SI base units and their associated, exact-valued constants.
//!
//! Unit | Unit symbol | Quantity | Associated constant | Associated constant name | Associated constant value
//! -|-|-|-|-|-
//! [SECOND] | $\text{s}$ | time | [DVCS] | Hyperfine transition frequency of Cs | $\Delta\\nu_\text{Cs}=9192631770\\,\text{Hz}$
//! [METER] | $\text{m}$ | length | [CLIGHT] | Speed of light | $c=299792458\\,\\frac{\text{m}}{\text{s}}$
//! [KILOGRAM] | $\text{kg}$ | mass | [PLANCK] | Planck constant | $h=6.62607015\times 10^{-34}\\,\text{J}\\cdot\text{s}$
//! [AMPERE] | $\text{A}$ | electric current | [QE] | Elementary charge | $e=1.602176634\\times 10^{-19}\\,\text{C}$
//! [KELVIN] | $\text{K}$ | thermodynamic temperature | [KB] | Boltzmann constant | $k_\text{B}=1.380649\times 10^{-23}\\,\\frac{\text{J}}{\text{K}}$
//! [MOL] | $\text{mol}$ | amount of substance | [NAV] | Avogadro constant | $N_\text{A}=6.02214076\times 10^{23}\\,\text{mol}^{-1}$
//! [CANDELA] | $\text{cd}$ | luminous intensity | [KCD] | Luminous efficacy of $540\\,\text{THz}$ radiation | $K_\text{cd}=683\\,\\frac{\text{lm}}{\text{W}}$
//!
//! ## Derived Units
//!
//! Additionally, it contains the following derived units.
//!
//! Unit | Unit symbol | Quantity | Definition
//! -|-|-|-
//! [HERTZ] | $\text{Hz}$ | frequency | $\text{s}^{-1}$
//! [NEWTON] | $\text{N}$ | force, weight | $\text{kg}\\frac{\text{m}}{\text{s}^2}$
//! [PASCAL] | $\text{Pa}$ | pressure, stress | $\\frac{\text{N}}{\text{m}^2}$
//! [JOULE] | $\text{J}$ | energy, work, heat | $\text{N}\text{m}$
//! [WATT] | $\text{W}$ | power, radiant flux | $\\frac{\text{J}}{\text{s}}$
//! [COULOMB] | $\text{C}$ | electric charge | $\text{A}\text{s}$
//! [VOLT] | $\text{V}$ | electrical potential difference | $\\frac{\text{W}}{\text{A}}$
//! [FARAD] | $\text{F}$ | capacitance | $\\frac{\text{C}}{\text{V}}$
//! [OHM] | $\text{Ω}$ | resistance, impedance, reactance | $\\frac{\text{V}}{\text{A}}$
//! [SIEMENS] | $\text{S}$ | electrical conductance | $\text{Ω}^{-1}$
//! [WEBER] | $\text{Wb}$ | magnetic flux | $\text{V}\text{s}$
//! [TESLA] | $\text{T}$ | magnetic flux density | $\\frac{\text{Wb}}{\text{m}^2}$
//! [HENRY] | $\text{H}$ | inductance | $\\frac{\text{Wb}}{\text{A}}$
//!
//! ## Additional units
//!
//! For convenience, a number of commonly used units that are not directly combinations of SI base units is also included.
//! These constants simplify the specification of properties, that are not given in SI units. However, as the representation
//! of quantities is unique, they do not appear in formatted outputs.
//!
//! Unit | Unit symbol | Quantity | Definition
//! -|-|-|-
//! [ANGSTROM] | $\text{\\AA}$ | length | $10^{-10}\\,\text{m}$
//! [AMU] | $\text{u}$ | mass | $1.6605390671738466\times 10^{-27}\\,\text{kg}$
//! [AU] | $\text{au}$ | length | $149597870700\\,\text{m}$
//! [BAR] | $\text{bar}$ | pressure | $10^5\\,\text{Pa}$
//! [CALORIE] | $\text{cal}$ | energy | $4.184\\,\text{J}$
//! [CELSIUS] | $^\\circ\text{C}$ | temperature | $t\\,^\\circ\text{C}=\\left(t+273.15\\right)\\,\text{K}$
//! [DAY] | $\text{d}$ | time | $86400\\,\text{s}$
//! [DEGREES] | $^\\circ$ | angle | $\\frac{\pi}{180}\\,\text{rad}$
//! [GRAM] | $\text{g}$ | mass | $10^{-3}\\,\text{kg}$
//! [HOUR] | $\text{h}$ | time | $3600\\,\text{s}$
//! [LITER] | $\text{l}$ | volume | $10^{-3}\\,\text{m}^3$
//! [MINUTE] | $\text{min}$ | time | $60\\,\text{s}$
//! [RADIANS] | $\text{rad}$ | angle |
//!
//! ## Additional constants
//!
//! Constant | Name | Symbol | Value
//! -|-|-|-
//! [G] | Gravitational constant | $G$ | $6.6743\\times 10^{-11}\\,\\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}$
//! [RGAS] | Ideal gas constant | $R=N_\text{Av}k_\text{B}$ | $8.31446261815324\\,\\frac{\text{J}}{\text{mol}\\cdot\text{K}}$
//!
//! ## Prefixes
//!
//! All units can be combined with the following prefixes:
//!
//! Prefix | Prefix symbol | value | | Prefix | Prefix symbol | value
//! -|-|-|-|-|-|-
//! [DECI] | $\text{d}$ | $10^{-1}$ | | [DECA] | $\text{da}$ | $10^{1}$
//! [CENTI] | $\text{c}$ | $10^{-2}$ | | [HECTO] | $\text{h}$ | $10^{2}$
//! [MILLI] | $\text{m}$ | $10^{-3}$ | | [KILO] | $\text{k}$ | $10^{3}$
//! [MICRO] | $\text{µ}$ | $10^{-6}$ | | [MEGA] | $\text{M}$ | $10^{6}$
//! [NANO] | $\text{n}$ | $10^{-9}$ | | [GIGA] | $\text{G}$ | $10^{9}$
//! [PICO] | $\text{p}$ | $10^{-12}$ | | [TERA] | $\text{T}$ | $10^{12}$
//! [FEMTO] | $\text{f}$ | $10^{-15}$ | | [PETA] | $\text{P}$ | $10^{15}$
//! [ATTO] | $\text{a}$ | $10^{-18}$ | | [EXA] | $\text{E}$ | $10^{18}$
//! [ZEPTO] | $\text{z}$ | $10^{-21}$ | | [ZETTA] | $\text{Z}$ | $10^{21}$
//! [YOCTO] | $\text{y}$ | $10^{-24}$ | | [YOTTA] | $\text{Y}$ | $10^{24}$
use super::{Quantity, QuantityError, Unit};
use ang::{Angle, Degrees, Radians};
use ndarray::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::hash::Hash;
use std::ops::Rem;
use std::ops::{Div, DivAssign, Mul, MulAssign};

/// Representation of a unit as a combination of SI base units.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SIUnit(pub(crate) [i8; 7]);

pub type SINumber = Quantity<f64, SIUnit>;
pub type SIArray<D> = Quantity<Array<f64, D>, SIUnit>;
pub type SIArray0 = SIArray<Ix0>;
pub type SIArray1 = SIArray<Ix1>;
pub type SIArray2 = SIArray<Ix2>;
pub type SIArray3 = SIArray<Ix3>;
pub type SIArray4 = SIArray<Ix4>;
pub type SIArray5 = SIArray<Ix5>;
pub type SIArray6 = SIArray<Ix6>;

impl Unit for SIUnit {
    const DIMENSIONLESS: Self = SIUnit([0; 7]);

    fn powi(&self, i: i32) -> Self {
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

    fn sqrt(&self) -> Result<Self, QuantityError> {
        self.root(2)
    }

    fn cbrt(&self) -> Result<Self, QuantityError> {
        self.root(3)
    }

    fn root(&self, i: i32) -> Result<Self, QuantityError> {
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
            Err(QuantityError::SINumberError {
                op: String::from("root"),
                cause: String::from("Unit exponents are not multiples of index"),
            })
        }
    }
}

impl Mul for SIUnit {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
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
    #[allow(clippy::suspicious_arithmetic_impl)]
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
    #[allow(clippy::suspicious_op_assign_impl)]
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
    #[allow(clippy::suspicious_op_assign_impl)]
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

const _METER: SIUnit = SIUnit([1, 0, 0, 0, 0, 0, 0]);
const _KILOGRAM: SIUnit = SIUnit([0, 1, 0, 0, 0, 0, 0]);
const _SECOND: SIUnit = SIUnit([0, 0, 1, 0, 0, 0, 0]);
const _AMPERE: SIUnit = SIUnit([0, 0, 0, 1, 0, 0, 0]);
const _MOL: SIUnit = SIUnit([0, 0, 0, 0, 1, 0, 0]);
const _KELVIN: SIUnit = SIUnit([0, 0, 0, 0, 0, 1, 0]);
const _CANDELA: SIUnit = SIUnit([0, 0, 0, 0, 0, 0, 1]);

const _HERTZ: SIUnit = SIUnit([0, 0, -1, 0, 0, 0, 0]);
const _NEWTON: SIUnit = SIUnit([1, 1, -2, 0, 0, 0, 0]);
const _JOULE: SIUnit = SIUnit([2, 1, -2, 0, 0, 0, 0]);
const _PASCAL: SIUnit = SIUnit([-1, 1, -2, 0, 0, 0, 0]);
const _WATT: SIUnit = SIUnit([2, 1, -3, 0, 0, 0, 0]);
const _CUBIC_METER: SIUnit = SIUnit([3, 0, 0, 0, 0, 0, 0]);
const _JOULE_PER_KELVIN: SIUnit = SIUnit([2, 1, -2, 0, 0, -1, 0]);
const _PER_MOL: SIUnit = SIUnit([0, 0, 0, 0, -1, 0, 0]);
const _JOULE_SECOND: SIUnit = SIUnit([2, 1, -1, 0, 0, 0, 0]);
const _JOULE_PER_MOL_AND_KELVIN: SIUnit = SIUnit([2, 1, -2, 0, -1, -1, 0]);
const _AMPERE_SECOND: SIUnit = SIUnit([0, 0, 1, 1, 0, 0, 0]);
const _VOLT: SIUnit = SIUnit([2, 1, -3, -1, 0, 0, 0]);
const _FARAD: SIUnit = SIUnit([-2, -1, 4, 2, 0, 0, 0]);
const _OHM: SIUnit = SIUnit([2, 1, -3, -2, 0, 0, 0]);
const _SIEMENS: SIUnit = SIUnit([-2, -1, 3, 2, 0, 0, 0]);
const _WEBER: SIUnit = SIUnit([2, 1, -2, -1, 0, 0, 0]);
const _TESLA: SIUnit = SIUnit([0, 1, -2, -1, 0, 0, 0]);
const _HENRY: SIUnit = SIUnit([2, 1, -2, -2, 0, 0, 0]);
const _METER_PER_SECOND: SIUnit = SIUnit([1, 0, -1, 0, 0, 0, 0]);
const _LUMEN_PER_WATT: SIUnit = SIUnit([-2, -1, 3, 0, 0, 0, 1]);

/// SI base unit meter $\\left(\text{m}\\right)$
pub const METER: SINumber = SINumber {
    unit: _METER,
    value: 1.0,
};
/// SI base unit kilogram $\\left(\text{kg}\\right)$
pub const KILOGRAM: SINumber = SINumber {
    unit: _KILOGRAM,
    value: 1.0,
};
/// SI base unit second $\\left(\text{s}\\right)$
pub const SECOND: SINumber = SINumber {
    unit: _SECOND,
    value: 1.0,
};
/// SI base unit ampere $\\left(\text{A}\\right)$
pub const AMPERE: SINumber = SINumber {
    unit: _AMPERE,
    value: 1.0,
};
/// SI base unit mol $\\left(\text{mol}\\right)$
pub const MOL: SINumber = SINumber {
    unit: _MOL,
    value: 1.0,
};
/// SI base unit kelvin $\\left(\text{K}\\right)$
pub const KELVIN: SINumber = SINumber {
    unit: _KELVIN,
    value: 1.0,
};
/// SI base unit candela $\\left(\text{cd}\\right)$
pub const CANDELA: SINumber = SINumber {
    unit: _CANDELA,
    value: 1.0,
};
/// Additional unit angstrom $\\left(1\\,\text{\\AA}=10^{-10}\\,\text{m}\\right)$
pub const ANGSTROM: SINumber = SINumber {
    unit: _METER,
    value: 1e-10,
};
/// Additional unit astronomical unit $\\left(1\\,\text{au}=149597870700\\,\text{m}\\right)$
pub const AU: SINumber = SINumber {
    unit: _METER,
    value: 149597870700.0,
};
/// Additional unit gram $\\left(1\\,\text{g}=10^{-3}\\,\text{kg}\\right)$
pub const GRAM: SINumber = SINumber {
    unit: _KILOGRAM,
    value: 1e-3,
};
/// Additional unit unified atomic mass $\\left(1\\,\text{u}\\approx 1.660539\\times 10^{-27}\\,\text{kg}\\right)$
pub const AMU: SINumber = SINumber {
    unit: _KILOGRAM,
    value: 1.6605390671738466e-27,
};
/// Derived unit hertz $\\left(1\\,\text{Hz}=1\\,\text{s}^{-1}\\right)$
pub const HERTZ: SINumber = SINumber {
    unit: _HERTZ,
    value: 1.0,
};
/// Derived unit newton $\\left(1\\,\text{N}=1\\,\text{kg}\\frac{\text{m}}{\text{s}^2}\\right)$
pub const NEWTON: SINumber = SINumber {
    unit: _NEWTON,
    value: 1.0,
};
/// Derived unit joule $\\left(1\\,\text{J}=1\\,\text{kg}\\frac{\text{m}^2}{\text{s}^2}\\right)$
pub const JOULE: SINumber = SINumber {
    unit: _JOULE,
    value: 1.0,
};
/// Derived unit pascal $\\left(1\\,\text{Pa}=1\\,\\frac{\text{kg}}{\text{m}\\cdot\text{s}^2}\\right)$
pub const PASCAL: SINumber = SINumber {
    unit: _PASCAL,
    value: 1.0,
};
/// Derived unit watt $\\left(1\\,\text{J}=1\\,\text{kg}\\frac{\text{m}^2}{\text{s}^3}\\right)$
pub const WATT: SINumber = SINumber {
    unit: _WATT,
    value: 1.0,
};
/// Additional unit bar $\\left(1\\,\text{bar}=10^5\\,\text{Pa}\\right)$
pub const BAR: SINumber = SINumber {
    unit: _PASCAL,
    value: 1e5,
};
/// Additional unit calorie $\\left(1\\,\text{cal}=4.184\\,\text{J}\\right)$
pub const CALORIE: SINumber = SINumber {
    unit: _JOULE,
    value: 4.184,
};
/// Additional unit liter $\\left(1\\,\text{l}=10^{-3}\\,\text{m}^3\\right)$
pub const LITER: SINumber = SINumber {
    unit: _CUBIC_METER,
    value: 1e-3,
};
/// Additional unit minute $\\left(1\\,\text{min}=60,\text{s}\\right)$
pub const MINUTE: SINumber = SINumber {
    unit: _SECOND,
    value: 60.0,
};
/// Additional unit hour $\\left(1\\,\text{h}=3600,\text{s}\\right)$
pub const HOUR: SINumber = SINumber {
    unit: _SECOND,
    value: 3600.0,
};
/// Additional unit day $\\left(1\\,\text{d}=86400,\text{s}\\right)$
pub const DAY: SINumber = SINumber {
    unit: _SECOND,
    value: 86400.0,
};
/// Derived unit coulomb $\\left(1\\,\text{C}=1\\,\text{A}\cdot\text{s}\\right)$
pub const COULOMB: SINumber = SINumber {
    unit: _AMPERE_SECOND,
    value: 1.0,
};
/// Derived unit volt $\\left(1\\,\text{V}=1\\,\\frac{\text{W}}{\text{A}}\\right)$
pub const VOLT: SINumber = SINumber {
    unit: _VOLT,
    value: 1.0,
};
/// Derived unit farad $\\left(1\\,\text{F}=1\\,\\frac{\text{C}}{\text{V}}\\right)$
pub const FARAD: SINumber = SINumber {
    unit: _FARAD,
    value: 1.0,
};
/// Derived unit ohm $\\left(1\\,\text{Ω}=1\\,\\frac{\text{V}}{\text{A}}\\right)$
pub const OHM: SINumber = SINumber {
    unit: _OHM,
    value: 1.0,
};
/// Derived unit siemens $\\left(1\\,\text{S}=1\\,\text{Ω}^{-1}\\right)$
pub const SIEMENS: SINumber = SINumber {
    unit: _SIEMENS,
    value: 1.0,
};
/// Derived unit weber $\\left(1\\,\text{Wb}=1\\,\text{V}\\cdot\text{s}\\right)$
pub const WEBER: SINumber = SINumber {
    unit: _WEBER,
    value: 1.0,
};
/// Derived unit tesla $\\left(1\\,\text{T}=1\\,\\frac{\text{Wb}}{\text{m}^2}\\right)$
pub const TESLA: SINumber = SINumber {
    unit: _TESLA,
    value: 1.0,
};
/// Derived unit henry $\\left(1\\,\text{T}=1\\,\\frac{\text{Wb}}{\text{A}}\\right)$
pub const HENRY: SINumber = SINumber {
    unit: _HENRY,
    value: 1.0,
};

/// Boltzmann constant $\\left(k_\text{B}=1.380649\times 10^{-23}\\,\\frac{\text{J}}{\text{K}}\\right)$
pub const KB: SINumber = SINumber {
    unit: _JOULE_PER_KELVIN,
    value: 1.380649e-23,
};
/// Avogadro constant $\\left(N_\text{A}=6.02214076\times 10^{23}\\,\text{mol}^{-1}\\right)$
pub const NAV: SINumber = SINumber {
    unit: _PER_MOL,
    value: 6.02214076e23,
};
/// Planck constant $\\left(h=6.62607015\times 10^{-34}\\,\text{J}\\cdot\text{s}\\right)$
pub const PLANCK: SINumber = SINumber {
    unit: _JOULE_SECOND,
    value: 6.62607015e-34,
};
/// Ideal gas constant $\\left(R=8.31446261815324\\,\\frac{\text{J}}{\text{mol}\\cdot\text{K}}\\right)$
pub const RGAS: SINumber = SINumber {
    unit: _JOULE_PER_MOL_AND_KELVIN,
    value: 1.380649e-23 * 6.02214076e23,
};
/// Hyperfine transition frequency of Cs $\\left(\Delta\\nu_\text{Cs}=9192631770\\,\text{Hz}\\right)$
pub const DVCS: SINumber = SINumber {
    unit: _HERTZ,
    value: 9192631770.0,
};
/// Elementary charge $\\left(e=1.602176634\\times 10^{-19}\\,\text{C}\\right)$
pub const QE: SINumber = SINumber {
    unit: _AMPERE_SECOND,
    value: 1.602176634e-19,
};
/// Speed of light $\\left(c=299792458\\,\\frac{\text{m}}{\text{s}}\\right)$
pub const CLIGHT: SINumber = SINumber {
    unit: _METER_PER_SECOND,
    value: 299792458.0,
};
/// Luminous efficacy of $540\\,\text{THz}$ radiation $\\left(K_\text{cd}=683\\,\\frac{\text{lm}}{\text{W}}\\right)$
pub const KCD: SINumber = SINumber {
    unit: _LUMEN_PER_WATT,
    value: 683.0,
};
/// Gravitational constant $\\left(G=6.6743\\times 10^{-11}\\,\\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}\\right)$
pub const G: SINumber = SINumber {
    unit: SIUnit([3, -1, -2, 0, 0, 0, 0]),
    value: 6.6743e-11,
};

/// Angle unit radian $\\left(\text{rad}\\right)$
pub const RADIANS: Angle = Radians(1.0);
/// Angle unit degree $\\left(1^\\circ=\frac{\pi}{180}\\,\text{rad}\\approx 0.0174532925\\,\text{rad}\\right)$
pub const DEGREES: Angle = Degrees(1.0);

/// Prefix yocto $\\left(\text{y}=10^{-24}\\right)$
pub const YOCTO: f64 = 1e-24;
/// Prefix zepto $\\left(\text{z}=10^{-21}\\right)$
pub const ZEPTO: f64 = 1e-21;
/// Prefix atto $\\left(\text{a}=10^{-18}\\right)$
pub const ATTO: f64 = 1e-18;
/// Prefix femto $\\left(\text{f}=10^{-15}\\right)$
pub const FEMTO: f64 = 1e-15;
/// Prefix pico $\\left(\text{p}=10^{-12}\\right)$
pub const PICO: f64 = 1e-12;
/// Prefix nano $\\left(\text{n}=10^{-9}\\right)$
pub const NANO: f64 = 1e-9;
/// Prefix micro $\\left(\text{µ}=10^{-6}\\right)$
pub const MICRO: f64 = 1e-6;
/// Prefix milli $\\left(\text{m}=10^{-3}\\right)$
pub const MILLI: f64 = 1e-3;
/// Prefix centi $\\left(\text{c}=10^{-2}\\right)$
pub const CENTI: f64 = 1e-2;
/// Prefix deci $\\left(\text{d}=10^{-1}\\right)$
pub const DECI: f64 = 1e-1;
/// Prefix deca $\\left(\text{da}=10^{1}\\right)$
pub const DECA: f64 = 1e1;
/// Prefix hecto $\\left(\text{h}=10^{2}\\right)$
pub const HECTO: f64 = 1e2;
/// Prefix kilo $\\left(\text{k}=10^{3}\\right)$
pub const KILO: f64 = 1e3;
/// Prefix mega $\\left(\text{M}=10^{6}\\right)$
pub const MEGA: f64 = 1e6;
/// Prefix giga $\\left(\text{G}=10^{9}\\right)$
pub const GIGA: f64 = 1e9;
/// Prefix tera $\\left(\text{T}=10^{12}\\right)$
pub const TERA: f64 = 1e12;
/// Prefix peta $\\left(\text{P}=10^{15}\\right)$
pub const PETA: f64 = 1e15;
/// Prefix exa $\\left(\text{E}=10^{18}\\right)$
pub const EXA: f64 = 1e18;
/// Prefix zetta $\\left(\text{Z}=10^{21}\\right)$
pub const ZETTA: f64 = 1e21;
/// Prefix yotta $\\left(\text{Y}=10^{24}\\right)$
pub const YOTTA: f64 = 1e24;

/// Additional unit celsius
pub struct CELSIUS;

impl Mul<CELSIUS> for f64 {
    type Output = SINumber;
    fn mul(self, _: CELSIUS) -> SINumber {
        SINumber {
            value: self + 273.15,
            unit: _KELVIN,
        }
    }
}

impl<S: Data<Elem = f64>, D: Dimension> Mul<CELSIUS> for ArrayBase<S, D> {
    type Output = SIArray<D>;
    fn mul(self, _: CELSIUS) -> SIArray<D> {
        SIArray {
            value: &self + 273.15,
            unit: _KELVIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn test_mul_si_si() {
        let mass = 1000.0 * KILOGRAM;
        let acc = 9.81 * METER / SECOND.powi(2);
        let force = mass * acc;
        assert_eq!(force.value, 1000.0 * 9.81);
        assert!(force.has_unit(&NEWTON))
    }

    #[test]
    fn test_mul_si_number() {
        let mass = 1000.0 * KILOGRAM;
        let acc = 9.81 * METER / SECOND.powi(2);
        let force = mass * acc / 1000.0;
        assert_eq!(force.value, 9.81);
        assert!(force.has_unit(&NEWTON))
    }

    #[test]
    fn test_add_si_si() {
        let p1 = 5.0 * BAR;
        let p2 = 1e5 * PASCAL;
        assert!((p1 + p2).has_unit(&BAR));
        assert_eq!((p1 + p2).value, 6e5)
    }

    #[test]
    #[should_panic]
    fn test_add_si_different_units() {
        let p = 5.0 * BAR;
        let t = 300.0 * KELVIN;
        let _r = p + t;
    }

    #[test]
    fn test_fmt() {
        let mass = 1000.0 * KILOGRAM;
        let acc = 9.81 * METER / SECOND.powi(2);
        let force = mass * acc;
        let format = fmt::format(format_args!("{}", force));
        let target = fmt::format(format_args!("{} kN", force.value / 1000.0));
        assert_eq!(format, target);
    }

    #[test]
    fn test_fmt_electro() {
        assert_eq!((2500. * WATT / AMPERE).to_string(), "2.5 kV".to_string());
        assert_eq!((2500. * COULOMB / VOLT).to_string(), "2.5 kF".to_string());
        assert_eq!((2500. * VOLT / AMPERE).to_string(), "2.5 kΩ".to_string());
        assert_eq!((2500. / OHM).to_string(), "2.5 kS".to_string());
        assert_eq!((2500. * VOLT * SECOND).to_string(), "2.5 kWb".to_string());
        assert_eq!(
            (2500. * WEBER / METER.powi(2)).to_string(),
            "2.5 kT".to_string()
        );
        assert_eq!((2500. * WEBER / AMPERE).to_string(), "2.5 kH".to_string());
    }

    #[test]
    fn test_celsius() {
        assert_eq!(25.0 * CELSIUS, 298.15 * KELVIN);
    }
}
