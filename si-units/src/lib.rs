#![warn(clippy::all)]
use ang::{Angle, Degrees, Radians};
use pyo3::prelude::*;
use pyo3::{exceptions::PyRuntimeError, PyErr};
use std::ops::{Add, Sub};
use thiserror::Error;

// #[macro_use]
// mod macros;

mod fmt;
mod si_unit;
pub use si_unit::SIUnit;
mod extra_units;
pub use extra_units::{Celsius, Debye, PyAngle};
mod sinumber;
pub use sinumber::SINumber;
// mod siarray;
// pub use siarray::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

/// Error type used to indicate unit conversion failures.
#[derive(Error, Debug)]
pub enum QuantityError {
    #[error("Inconsistent units {unit1} and {unit2}")]
    InconsistentUnits { unit1: SIUnit, unit2: SIUnit },
    #[error("Unit exponents are not multiples of index")]
    InvalidRoot,
    #[error("Debye can only be raised to even powers")]
    DebyePower,
}

impl From<QuantityError> for PyErr {
    fn from(e: QuantityError) -> PyErr {
        PyRuntimeError::new_err(e.to_string())
    }
}

fn try_add<T1, T2>(
    value1: T1,
    unit1: SIUnit,
    value2: T2,
    unit2: SIUnit,
) -> Result<(<T1 as Add<T2>>::Output, SIUnit), QuantityError>
where
    T1: Add<T2>,
{
    if unit1 == unit2 {
        Ok((value1 + value2, unit1))
    } else {
        Err(QuantityError::InconsistentUnits { unit1, unit2 })
    }
}

fn try_sub<T1, T2>(
    value1: T1,
    unit1: SIUnit,
    value2: T2,
    unit2: SIUnit,
) -> Result<(<T1 as Sub<T2>>::Output, SIUnit), QuantityError>
where
    T1: Sub<T2>,
{
    if unit1 == unit2 {
        Ok((value1 - value2, unit1))
    } else {
        Err(QuantityError::InconsistentUnits { unit1, unit2 })
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
/// Additional unit celsius
const CELSIUS: Celsius = Celsius;
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
// /// Additional unit debye $\\left(1\\,\text{De}^2=10^{-19}\\,\text{J\\AA}^3\\right)$
// pub const DEBYE: Debye = Debye(1.0);

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

/// Prefix quecto $\\left(\text{q}=10^{-30}\\right)$
pub const QUECTO: f64 = 1e-30;
/// Prefix ronto $\\left(\text{r}=10^{-27}\\right)$
pub const RONTO: f64 = 1e-27;
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
/// Prefix ronna $\\left(\text{R}=10^{27}\\right)$
pub const RONNA: f64 = 1e27;
/// Prefix quetta $\\left(\text{Q}=10^{30}\\right)$
pub const QUETTA: f64 = 1e30;

#[pymodule]
pub fn si_units(_py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<SINumber>()?;
    // m.add_class::<PySIArray1>()?;
    // m.add_class::<PySIArray2>()?;
    // m.add_class::<PySIArray3>()?;
    // m.add_class::<PySIArray4>()?;

    m.add("SECOND", SECOND)?;
    m.add("METER", METER)?;
    m.add("KILOGRAM", KILOGRAM)?;
    m.add("AMPERE", AMPERE)?;
    m.add("KELVIN", KELVIN)?;
    m.add("MOL", MOL)?;
    m.add("CANDELA", CANDELA)?;

    m.add("DVCS", DVCS)?;
    m.add("CLIGHT", CLIGHT)?;
    m.add("PLANCK", PLANCK)?;
    m.add("QE", QE)?;
    m.add("KB", KB)?;
    m.add("NAV", NAV)?;
    m.add("KCD", KCD)?;

    m.add("HERTZ", HERTZ)?;
    m.add("NEWTON", NEWTON)?;
    m.add("PASCAL", PASCAL)?;
    m.add("JOULE", JOULE)?;
    m.add("WATT", WATT)?;
    m.add("COULOMB", COULOMB)?;
    m.add("VOLT", VOLT)?;
    m.add("FARAD", FARAD)?;
    m.add("OHM", OHM)?;
    m.add("SIEMENS", SIEMENS)?;
    m.add("WEBER", WEBER)?;
    m.add("TESLA", TESLA)?;
    m.add("HENRY", HENRY)?;

    m.add("ANGSTROM", ANGSTROM)?;
    m.add("AMU", AMU)?;
    m.add("AU", AU)?;
    m.add("BAR", BAR)?;
    m.add("CALORIE", CALORIE)?;
    m.add("CELSIUS", CELSIUS)?;
    m.add("DAY", DAY)?;
    // m.add("DEBYE", DEBYE)?;
    // m.add("DEGREES", DEGREES)?;
    m.add("GRAM", GRAM)?;
    m.add("HOUR", HOUR)?;
    m.add("LITER", LITER)?;
    m.add("MINUTE", MINUTE)?;
    // m.add("RADIANS", RADIANS)?;

    m.add("G", G)?;
    m.add("RGAS", RGAS)?;

    m.add("QUECTO", QUECTO)?;
    m.add("RONTO", RONTO)?;
    m.add("YOCTO", YOCTO)?;
    m.add("ZEPTO", ZEPTO)?;
    m.add("ATTO", ATTO)?;
    m.add("FEMTO", FEMTO)?;
    m.add("PICO", PICO)?;
    m.add("NANO", NANO)?;
    m.add("MICRO", MICRO)?;
    m.add("MILLI", MILLI)?;
    m.add("CENTI", CENTI)?;
    m.add("DECI", DECI)?;
    m.add("DECA", DECA)?;
    m.add("HECTO", HECTO)?;
    m.add("KILO", KILO)?;
    m.add("MEGA", MEGA)?;
    m.add("GIGA", GIGA)?;
    m.add("TERA", TERA)?;
    m.add("PETA", PETA)?;
    m.add("EXA", EXA)?;
    m.add("ZETTA", ZETTA)?;
    m.add("YOTTA", YOTTA)?;
    m.add("RONNA", RONNA)?;
    m.add("QUETTA", QUETTA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::SIArray1;
    // use std::fmt;

    // #[test]
    // fn test_mul_si_si() {
    //     let mass = 1000.0 * KILOGRAM;
    //     let acc = 9.81 * METER / SECOND.powi(2);
    //     let force = mass * acc;
    //     assert_eq!(force.value, 1000.0 * 9.81);
    //     assert!(force.has_unit(NEWTON))
    // }

    // #[test]
    // fn test_mul_si_number() {
    //     let mass = 1000.0 * KILOGRAM;
    //     let acc = 9.81 * METER / SECOND.powi(2);
    //     let force = mass * acc / 1000.0;
    //     assert_eq!(force.value, 9.81);
    //     assert!(force.has_unit(NEWTON))
    // }

    // #[test]
    // fn test_add_si_si() {
    //     let p1 = 5.0 * BAR;
    //     let p2 = 1e5 * PASCAL;
    //     assert!((p1 + p2).has_unit(&BAR));
    //     assert_eq!((p1 + p2).value, 6e5)
    // }

    // #[test]
    // #[should_panic]
    // fn test_add_si_different_units() {
    //     let p = 5.0 * BAR;
    //     let t = 300.0 * KELVIN;
    //     let _r = p + t;
    // }

    // #[test]
    // fn test_fmt() {
    //     let mass = 1000.0 * KILOGRAM;
    //     let acc = 9.81 * METER / SECOND.powi(2);
    //     let force = mass * acc;
    //     let format = fmt::format(format_args!("{}", force));
    //     let target = fmt::format(format_args!("{} kN", force.value / 1000.0));
    //     assert_eq!(format, target);
    // }

    // #[test]
    // fn test_fmt_electro() {
    //     assert_eq!((2500. * WATT / AMPERE).to_string(), "2.5 kV".to_string());
    //     assert_eq!((2500. * COULOMB / VOLT).to_string(), "2.5 kF".to_string());
    //     assert_eq!((2500. * VOLT / AMPERE).to_string(), "2.5 kΩ".to_string());
    //     assert_eq!((2500. / OHM).to_string(), "2.5 kS".to_string());
    //     assert_eq!((2500. * VOLT * SECOND).to_string(), "2.5 kWb".to_string());
    //     assert_eq!(
    //         (2500. * WEBER / METER.powi(2)).to_string(),
    //         "2.5 kT".to_string()
    //     );
    //     assert_eq!((2500. * WEBER / AMPERE).to_string(), "2.5 kH".to_string());
    // }

    // #[test]
    // fn test_celsius() {
    //     assert_eq!(25.0 * CELSIUS, 298.15 * KELVIN);
    //     assert_eq!(298.15 * KELVIN / CELSIUS, 25.0);
    // }

    // #[test]
    // fn test_debye() {
    //     assert_eq!((4.0 * DEBYE).powi(2), 16e-19 * JOULE * ANGSTROM.powi(3));
    // }

    // #[test]
    // fn test_from_shape_fn() {
    //     let arr = SIArray1::from_shape_fn(3, |i| i as f64 * KELVIN);
    //     assert_eq!(arr.to_reduced(KELVIN).unwrap(), arr1(&[0.0, 1.0, 2.0]));
    // }

    // #[test]
    // #[should_panic(expected = "Inconsistent units Pa and K")]
    // fn test_from_vec() {
    //     let vec = vec![3.0 * PASCAL, 0.04 * BAR, 7.0 * JOULE / METER.powi(3)];
    //     let arr = SIArray1::from_vec(vec);
    //     assert_eq!(arr.to_reduced(PASCAL).unwrap(), arr1(&[3.0, 4000.0, 7.0]));
    //     SIArray1::from_vec(vec![BAR, KELVIN]);
    // }
}
