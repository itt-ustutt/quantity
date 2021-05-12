use crate::QuantityError;
use pyo3::prelude::*;
use pyo3::{exceptions::PyRuntimeError, PyErr};

#[macro_use]
mod macros;

mod sinumber;
pub use sinumber::PySINumber;
mod siarray;
pub use siarray::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};
mod angle;
pub use angle::PyAngle;

impl From<QuantityError> for PyErr {
    fn from(e: QuantityError) -> PyErr {
        PyRuntimeError::new_err(e.to_string())
    }
}

const METER: PySINumber = PySINumber {
    _data: crate::si::METER,
};
const KILOGRAM: PySINumber = PySINumber {
    _data: crate::si::KILOGRAM,
};
const SECOND: PySINumber = PySINumber {
    _data: crate::si::SECOND,
};
const AMPERE: PySINumber = PySINumber {
    _data: crate::si::AMPERE,
};
const MOL: PySINumber = PySINumber {
    _data: crate::si::MOL,
};
const KELVIN: PySINumber = PySINumber {
    _data: crate::si::KELVIN,
};
const CANDELA: PySINumber = PySINumber {
    _data: crate::si::CANDELA,
};
const ANGSTROM: PySINumber = PySINumber {
    _data: crate::si::ANGSTROM,
};
const GRAM: PySINumber = PySINumber {
    _data: crate::si::GRAM,
};
const HERTZ: PySINumber = PySINumber {
    _data: crate::si::HERTZ,
};
const NEWTON: PySINumber = PySINumber {
    _data: crate::si::NEWTON,
};
const PASCAL: PySINumber = PySINumber {
    _data: crate::si::PASCAL,
};
const BAR: PySINumber = PySINumber {
    _data: crate::si::BAR,
};
const JOULE: PySINumber = PySINumber {
    _data: crate::si::JOULE,
};
const CALORIE: PySINumber = PySINumber {
    _data: crate::si::CALORIE,
};
const WATT: PySINumber = PySINumber {
    _data: crate::si::WATT,
};
const LITER: PySINumber = PySINumber {
    _data: crate::si::LITER,
};
const RADIANS: PyAngle = PyAngle {
    _data: crate::si::RADIANS,
};
const DEGREES: PyAngle = PyAngle {
    _data: crate::si::DEGREES,
};

/// Boltzmann constant.
const KB: PySINumber = PySINumber {
    _data: crate::si::KB,
};

/// Avogadro constant.
const NAV: PySINumber = PySINumber {
    _data: crate::si::NAV,
};

/// Planck constant.
const PLANCK: PySINumber = PySINumber {
    _data: crate::si::PLANCK,
};

/// Universal gas constant.
const RGAS: PySINumber = PySINumber {
    _data: crate::si::RGAS,
};

/// Implementation of SI numbers.
#[pymodule]
pub fn si(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySINumber>()?;
    m.add_class::<PySIArray1>()?;
    m.add_class::<PySIArray2>()?;
    m.add_class::<PySIArray3>()?;
    m.add_class::<PySIArray4>()?;

    m.add("METER", METER)?;
    m.add("KILOGRAM", KILOGRAM)?;
    m.add("SECOND", SECOND)?;
    m.add("AMPERE", AMPERE)?;
    m.add("MOL", MOL)?;
    m.add("KELVIN", KELVIN)?;
    m.add("CANDELA", CANDELA)?;
    m.add("ANGSTROM", ANGSTROM)?;
    m.add("GRAM", GRAM)?;
    m.add("HERTZ", HERTZ)?;
    m.add("NEWTON", NEWTON)?;
    m.add("PASCAL", PASCAL)?;
    m.add("BAR", BAR)?;
    m.add("JOULE", JOULE)?;
    m.add("CALORIE", CALORIE)?;
    m.add("WATT", WATT)?;
    m.add("LITER", LITER)?;
    m.add("RADIANS", RADIANS)?;
    m.add("DEGREES", DEGREES)?;
    m.add("KB", KB)?;
    m.add("NAV", NAV)?;
    m.add("PLANCK", PLANCK)?;
    m.add("RGAS", RGAS)?;

    m.add("YOCTO", crate::si::YOCTO)?;
    m.add("ZEPTO", crate::si::ZEPTO)?;
    m.add("ATTO", crate::si::ATTO)?;
    m.add("FEMTO", crate::si::FEMTO)?;
    m.add("PICO", crate::si::PICO)?;
    m.add("NANO", crate::si::NANO)?;
    m.add("MICRO", crate::si::MICRO)?;
    m.add("MILLI", crate::si::MILLI)?;
    m.add("CENTI", crate::si::CENTI)?;
    m.add("DECI", crate::si::DECI)?;
    m.add("DECA", crate::si::DECA)?;
    m.add("HECTO", crate::si::HECTO)?;
    m.add("KILO", crate::si::KILO)?;
    m.add("MEGA", crate::si::MEGA)?;
    m.add("GIGA", crate::si::GIGA)?;
    m.add("TERA", crate::si::TERA)?;
    m.add("PETA", crate::si::PETA)?;
    m.add("EXA", crate::si::EXA)?;
    m.add("ZETTA", crate::si::ZETTA)?;
    m.add("YOTTA", crate::si::YOTTA)?;
    Ok(())
}
