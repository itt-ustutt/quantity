use pyo3::prelude::*;
use quantity::python::*;

/// Implementation of SI numbers.
#[pymodule]
pub fn si_units(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
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

    m.add("YOCTO", quantity::si::YOCTO)?;
    m.add("ZEPTO", quantity::si::ZEPTO)?;
    m.add("ATTO", quantity::si::ATTO)?;
    m.add("FEMTO", quantity::si::FEMTO)?;
    m.add("PICO", quantity::si::PICO)?;
    m.add("NANO", quantity::si::NANO)?;
    m.add("MICRO", quantity::si::MICRO)?;
    m.add("MILLI", quantity::si::MILLI)?;
    m.add("CENTI", quantity::si::CENTI)?;
    m.add("DECI", quantity::si::DECI)?;
    m.add("DECA", quantity::si::DECA)?;
    m.add("HECTO", quantity::si::HECTO)?;
    m.add("KILO", quantity::si::KILO)?;
    m.add("MEGA", quantity::si::MEGA)?;
    m.add("GIGA", quantity::si::GIGA)?;
    m.add("TERA", quantity::si::TERA)?;
    m.add("PETA", quantity::si::PETA)?;
    m.add("EXA", quantity::si::EXA)?;
    m.add("ZETTA", quantity::si::ZETTA)?;
    m.add("YOTTA", quantity::si::YOTTA)?;
    Ok(())
}
