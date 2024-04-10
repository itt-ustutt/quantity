use crate::QuantityError;
use pyo3::prelude::*;
use pyo3::{exceptions::PyRuntimeError, PyErr};

#[macro_use]
mod macros;

mod sinumber;
pub use sinumber::{PyCelsius, PyDebye, PySINumber};
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
const SECOND: PySINumber = PySINumber(crate::si::SECOND);
const METER: PySINumber = PySINumber(crate::si::METER);
const KILOGRAM: PySINumber = PySINumber(crate::si::KILOGRAM);
const AMPERE: PySINumber = PySINumber(crate::si::AMPERE);
const MOL: PySINumber = PySINumber(crate::si::MOL);
const KELVIN: PySINumber = PySINumber(crate::si::KELVIN);
const CANDELA: PySINumber = PySINumber(crate::si::CANDELA);

// Associated Constants
const DVCS: PySINumber = PySINumber(crate::si::DVCS);
const CLIGHT: PySINumber = PySINumber(crate::si::CLIGHT);
const PLANCK: PySINumber = PySINumber(crate::si::PLANCK);
const QE: PySINumber = PySINumber(crate::si::QE);
const KB: PySINumber = PySINumber(crate::si::KB);
const NAV: PySINumber = PySINumber(crate::si::NAV);
const KCD: PySINumber = PySINumber(crate::si::KCD);

// Derived Units
const HERTZ: PySINumber = PySINumber(crate::si::HERTZ);
const NEWTON: PySINumber = PySINumber(crate::si::NEWTON);
const PASCAL: PySINumber = PySINumber(crate::si::PASCAL);
const JOULE: PySINumber = PySINumber(crate::si::JOULE);
const WATT: PySINumber = PySINumber(crate::si::WATT);
const COULOMB: PySINumber = PySINumber(crate::si::COULOMB);
const VOLT: PySINumber = PySINumber(crate::si::VOLT);
const FARAD: PySINumber = PySINumber(crate::si::FARAD);
const OHM: PySINumber = PySINumber(crate::si::OHM);
const SIEMENS: PySINumber = PySINumber(crate::si::SIEMENS);
const WEBER: PySINumber = PySINumber(crate::si::WEBER);
const TESLA: PySINumber = PySINumber(crate::si::TESLA);
const HENRY: PySINumber = PySINumber(crate::si::HENRY);

// Additional Units
const ANGSTROM: PySINumber = PySINumber(crate::si::ANGSTROM);
const AMU: PySINumber = PySINumber(crate::si::AMU);
const AU: PySINumber = PySINumber(crate::si::AU);
const BAR: PySINumber = PySINumber(crate::si::BAR);
const CALORIE: PySINumber = PySINumber(crate::si::CALORIE);
const CELSIUS: PyCelsius = PyCelsius;
const DAY: PySINumber = PySINumber(crate::si::DAY);
const DEBYE: PyDebye = PyDebye(crate::si::DEBYE);
const DEGREES: PyAngle = PyAngle(crate::si::DEGREES);
const GRAM: PySINumber = PySINumber(crate::si::GRAM);
const HOUR: PySINumber = PySINumber(crate::si::HOUR);
const LITER: PySINumber = PySINumber(crate::si::LITER);
const MINUTE: PySINumber = PySINumber(crate::si::MINUTE);
const RADIANS: PyAngle = PyAngle(crate::si::RADIANS);

// Additional Constants
const G: PySINumber = PySINumber(crate::si::G);
const RGAS: PySINumber = PySINumber(crate::si::RGAS);

#[pymodule]
pub fn quantity(_py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
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
    m.add("DEBYE", DEBYE)?;
    m.add("DEGREES", DEGREES)?;
    m.add("GRAM", GRAM)?;
    m.add("HOUR", HOUR)?;
    m.add("LITER", LITER)?;
    m.add("MINUTE", MINUTE)?;
    m.add("RADIANS", RADIANS)?;

    m.add("G", G)?;
    m.add("RGAS", RGAS)?;

    m.add("QUECTO", crate::si::QUECTO)?;
    m.add("RONTO", crate::si::RONTO)?;
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
    m.add("RONNA", crate::si::RONNA)?;
    m.add("QUETTA", crate::si::QUETTA)?;
    Ok(())
}
