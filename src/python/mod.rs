use crate::QuantityError;
use pyo3::prelude::*;
use pyo3::{exceptions::PyRuntimeError, PyErr};

#[macro_use]
mod macros;

mod sinumber;
pub use sinumber::{PyCelsius, PySINumber};
mod siarray;
pub use siarray::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};
mod angle;
pub use angle::PyAngle;

impl From<QuantityError> for PyErr {
    fn from(e: QuantityError) -> PyErr {
        PyRuntimeError::new_err(e.to_string())
    }
}

// Base Units
pub const SECOND: PySINumber = PySINumber(crate::si::SECOND);
pub const METER: PySINumber = PySINumber(crate::si::METER);
pub const KILOGRAM: PySINumber = PySINumber(crate::si::KILOGRAM);
pub const AMPERE: PySINumber = PySINumber(crate::si::AMPERE);
pub const MOL: PySINumber = PySINumber(crate::si::MOL);
pub const KELVIN: PySINumber = PySINumber(crate::si::KELVIN);
pub const CANDELA: PySINumber = PySINumber(crate::si::CANDELA);

// Associated Constants
pub const DVCS: PySINumber = PySINumber(crate::si::DVCS);
pub const CLIGHT: PySINumber = PySINumber(crate::si::CLIGHT);
pub const PLANCK: PySINumber = PySINumber(crate::si::PLANCK);
pub const QE: PySINumber = PySINumber(crate::si::QE);
pub const KB: PySINumber = PySINumber(crate::si::KB);
pub const NAV: PySINumber = PySINumber(crate::si::NAV);
pub const KCD: PySINumber = PySINumber(crate::si::KCD);

// Derived Units
pub const HERTZ: PySINumber = PySINumber(crate::si::HERTZ);
pub const NEWTON: PySINumber = PySINumber(crate::si::NEWTON);
pub const PASCAL: PySINumber = PySINumber(crate::si::PASCAL);
pub const JOULE: PySINumber = PySINumber(crate::si::JOULE);
pub const WATT: PySINumber = PySINumber(crate::si::WATT);
pub const COULOMB: PySINumber = PySINumber(crate::si::COULOMB);
pub const VOLT: PySINumber = PySINumber(crate::si::VOLT);
pub const FARAD: PySINumber = PySINumber(crate::si::FARAD);
pub const OHM: PySINumber = PySINumber(crate::si::OHM);
pub const SIEMENS: PySINumber = PySINumber(crate::si::SIEMENS);
pub const WEBER: PySINumber = PySINumber(crate::si::WEBER);
pub const TESLA: PySINumber = PySINumber(crate::si::TESLA);
pub const HENRY: PySINumber = PySINumber(crate::si::HENRY);

// Additional Units
pub const ANGSTROM: PySINumber = PySINumber(crate::si::ANGSTROM);
pub const AMU: PySINumber = PySINumber(crate::si::AMU);
pub const AU: PySINumber = PySINumber(crate::si::AU);
pub const BAR: PySINumber = PySINumber(crate::si::BAR);
pub const CALORIE: PySINumber = PySINumber(crate::si::CALORIE);
pub const CELSIUS: PyCelsius = PyCelsius;
pub const DAY: PySINumber = PySINumber(crate::si::DAY);
pub const DEGREES: PyAngle = PyAngle(crate::si::DEGREES);
pub const GRAM: PySINumber = PySINumber(crate::si::GRAM);
pub const HOUR: PySINumber = PySINumber(crate::si::HOUR);
pub const LITER: PySINumber = PySINumber(crate::si::LITER);
pub const MINUTE: PySINumber = PySINumber(crate::si::MINUTE);
pub const RADIANS: PyAngle = PyAngle(crate::si::RADIANS);

// Additional Constants
pub const G: PySINumber = PySINumber(crate::si::G);
pub const RGAS: PySINumber = PySINumber(crate::si::RGAS);

/// Implementation of SI numbers.
#[pymodule]
pub fn quantity(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<PySINumber>()?;
    m.add_class::<PySIArray1>()?;
    m.add_class::<PySIArray2>()?;
    m.add_class::<PySIArray3>()?;
    m.add_class::<PySIArray4>()?;

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
    m.add("DEGREES", DEGREES)?;
    m.add("GRAM", GRAM)?;
    m.add("HOUR", HOUR)?;
    m.add("LITER", LITER)?;
    m.add("MINUTE", MINUTE)?;
    m.add("RADIANS", RADIANS)?;

    m.add("G", G)?;
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
