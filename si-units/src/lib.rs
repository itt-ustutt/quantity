#![warn(clippy::all)]
#![allow(non_snake_case)]
use ndarray::{arr1, Array1};
use numpy::IntoPyArray;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyNotImplemented};
use pyo3::{PyErr, PyTypeInfo};
use thiserror::Error;

mod si_unit;
use si_unit::SIUnit;
mod extra_units;
use extra_units::{Angle, Celsius, Debye};
mod fmt;

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

#[pyclass(name = "SIObject", module = "si_units._core", frozen)]
pub struct PySIObject {
    value: PyObject,
    unit: SIUnit,
}

impl PySIObject {
    fn new(value: PyObject, unit: SIUnit) -> Self {
        Self { value, unit }
    }

    fn new_unit_checked(value: Bound<'_, PyAny>, unit: SIUnit) -> PyResult<Bound<'_, PyAny>> {
        Ok(if unit.is_dimensionless() {
            value
        } else {
            Bound::new(value.py(), Self::new(value.unbind(), unit))?.into_any()
        })
    }

    fn check_units(&self, other: &Self) -> PyResult<SIUnit> {
        if self.unit == other.unit {
            Ok(self.unit)
        } else {
            Err(QuantityError::InconsistentUnits {
                unit1: self.unit,
                unit2: other.unit,
            })?
        }
    }
}

impl From<QuantityError> for PyErr {
    fn from(e: QuantityError) -> PyErr {
        PyRuntimeError::new_err(e.to_string())
    }
}

#[pymethods]
impl PySIObject {
    #[new]
    pub fn py_new(value: Bound<'_, PyAny>, unit: [i8; 7]) -> Self {
        Self::new(value.unbind(), SIUnit(unit))
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> (&Bound<'py, PyAny>, [i8; 7]) {
        (self.value.bind(py), self.unit.0)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        if let Ok(v) = self.value.extract::<f64>(py) {
            Ok(SINumber::new(v, self.unit).to_string())
        } else {
            let value = self
                .value
                .call_method0(py, "__repr__")?
                .extract::<String>(py)?;
            Ok(format!("{} {}", value, self.unit))
        }
    }

    fn _repr_latex_(&self, py: Python) -> Option<String> {
        self.value
            .extract::<f64>(py)
            .map(|v| format!("${}$", SINumber::new(v, self.unit).to_latex()))
            .ok()
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyResult<PyObject> {
        self.check_units(other).and_then(|_| match op {
            CompareOp::Eq => self.value.call_method1(py, "__eq__", (&other.value,)),
            CompareOp::Ne => self.value.call_method1(py, "__ne__", (&other.value,)),
            CompareOp::Lt => self.value.call_method1(py, "__lt__", (&other.value,)),
            CompareOp::Le => self.value.call_method1(py, "__le__", (&other.value,)),
            CompareOp::Gt => self.value.call_method1(py, "__gt__", (&other.value,)),
            CompareOp::Ge => self.value.call_method1(py, "__ge__", (&other.value,)),
        })
    }

    pub fn sqrt(&self, py: Python) -> PyResult<Self> {
        let value = if let Ok(v) = self.value.extract::<f64>(py) {
            PyFloat::new(py, v.sqrt()).into_any().unbind()
        } else {
            self.value.call_method0(py, "sqrt")?
        };
        Ok(Self::new(value, self.unit.sqrt()?))
    }

    pub fn cbrt(&self, py: Python) -> PyResult<Self> {
        let value = if let Ok(v) = self.value.extract::<f64>(py) {
            PyFloat::new(py, v.cbrt()).into_any().unbind()
        } else {
            self.value.call_method0(py, "cbrt")?
        };
        Ok(Self::new(value, self.unit.cbrt()?))
    }

    pub fn has_unit(&self, other: PyRef<'_, Self>) -> bool {
        self.unit.eq(&other.unit)
    }

    #[classattr]
    fn __array_priority__() -> u64 {
        1000
    }

    fn __add__(&self, py: Python, rhs: &Self) -> PyResult<Self> {
        self.check_units(rhs).and_then(|unit| {
            Ok(Self::new(
                self.value.call_method1(py, "__add__", (&rhs.value,))?,
                unit,
            ))
        })
    }

    fn __sub__(&self, py: Python, rhs: &Self) -> PyResult<Self> {
        self.check_units(rhs).and_then(|unit| {
            Ok(Self::new(
                self.value.call_method1(py, "__sub__", (&rhs.value,))?,
                unit,
            ))
        })
    }

    fn __mul__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let (rhs_value, unit) = if let Ok(r) = rhs.downcast::<Self>().map(Bound::get) {
            (r.value.bind(rhs.py()).clone(), self.unit * r.unit)
        } else {
            (rhs.clone(), self.unit)
        };
        let mut value = self
            .value
            .bind(rhs.py())
            .call_method1("__mul__", (&rhs_value,))?;
        if PyNotImplemented::is_exact_type_of(&value) {
            value = rhs_value.call_method1("__rmul__", (&self.value,))?;
        }
        Self::new_unit_checked(value, unit)
    }

    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let (lhs_value, unit) = if let Ok(l) = lhs.downcast::<Self>().map(Bound::get) {
            (l.value.bind(lhs.py()).clone(), l.unit * self.unit)
        } else {
            (lhs.clone(), self.unit)
        };
        let mut value = self
            .value
            .bind(lhs.py())
            .call_method1("__rmul__", (&lhs_value,))?;
        if PyNotImplemented::is_exact_type_of(&value) {
            value = lhs_value.call_method1("__mul__", (&self.value,))?;
        }
        Self::new_unit_checked(value, unit)
    }

    fn __truediv__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let (rhs_value, unit) = if let Ok(r) = rhs.downcast::<Self>().map(Bound::get) {
            (r.value.bind(rhs.py()).clone(), self.unit / r.unit)
        } else if rhs.downcast::<Celsius>().is_ok() {
            return if self.unit == _KELVIN {
                let delta = PyFloat::new(rhs.py(), 273.15);
                self.value.bind(rhs.py()).call_method1("__sub__", (&delta,))
            } else {
                Err(QuantityError::InconsistentUnits {
                    unit1: self.unit,
                    unit2: _KELVIN,
                })?
            };
        } else {
            (rhs.clone(), self.unit)
        };
        let mut value = self
            .value
            .bind(rhs.py())
            .call_method1("__truediv__", (&rhs_value,))?;
        if PyNotImplemented::is_exact_type_of(&value) {
            value = rhs_value.call_method1("__rtruediv__", (&self.value,))?;
        }
        Self::new_unit_checked(value, unit)
    }

    fn __rtruediv__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let (lhs_value, unit) = if let Ok(l) = lhs.downcast::<Self>().map(Bound::get) {
            (l.value.bind(lhs.py()).clone(), l.unit / self.unit)
        } else {
            (lhs.clone(), self.unit.recip())
        };
        let mut value = self
            .value
            .bind(lhs.py())
            .call_method1("__rtruediv__", (&lhs_value,))?;
        if PyNotImplemented::is_exact_type_of(&value) {
            value = lhs_value.call_method1("__truediv__", (&self.value,))?;
        }
        Self::new_unit_checked(value, unit)
    }

    fn __pow__(&self, py: Python, i: i32, _mod: Option<u32>) -> PyResult<Self> {
        let value = self.value.call_method1(py, "__pow__", (i,))?;
        let unit = self.unit.powi(i);
        Ok(Self::new(value, unit))
    }

    fn __neg__(&self, py: Python) -> PyResult<Self> {
        let value = self.value.call_method0(py, "__neg__")?;
        Ok(Self::new(value, self.unit))
    }

    fn __abs__(&self, py: Python) -> PyResult<Self> {
        let value = self.value.call_method0(py, "__abs__")?;
        Ok(Self::new(value, self.unit))
    }

    #[getter]
    fn get_shape(&self, py: Python) -> PyResult<PyObject> {
        self.value.getattr(py, "shape")
    }

    fn __len__(&self, py: Python) -> PyResult<usize> {
        self.value
            .call_method0(py, "__len__")
            .and_then(|v| v.extract::<usize>(py))
    }

    fn __getitem__(&self, py: Python, idx: isize) -> PyResult<Self> {
        let value = self.value.call_method1(py, "__getitem__", (idx,))?;
        Ok(Self::new(value, self.unit))
    }

    fn __setitem__(&self, py: Python, idx: isize, value: SINumber) -> PyResult<()> {
        if self.unit == value.unit {
            self.value
                .call_method1(py, "__setitem__", (idx, value.value))?;
            Ok(())
        } else {
            Err(QuantityError::InconsistentUnits {
                unit1: self.unit,
                unit2: value.unit,
            })?
        }
    }
}

#[derive(Clone, Copy)]
pub struct SIObject<T> {
    value: T,
    unit: SIUnit,
}

pub type SINumber = SIObject<f64>;

impl<T> SIObject<T> {
    pub const fn new(value: T, unit: SIUnit) -> Self {
        Self { value, unit }
    }
}

impl<'py> FromPyObject<'py> for SINumber {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = ob.py();
        let ob = ob.downcast::<PySIObject>()?.borrow();
        let value = ob.value.extract::<f64>(py)?;
        let unit = ob.unit;
        Ok(SINumber { value, unit })
    }
}

#[pyfunction]
fn array1<'py>(value: Bound<'py, PyAny>) -> PyResult<Bound<'py, PySIObject>> {
    let py = value.py();
    if let Ok(v) = value.extract::<SINumber>() {
        let value = arr1(&[1.0]) * v.value;
        let value = value.into_pyarray(py).into_any().unbind();
        Bound::new(py, PySIObject::new(value, v.unit))
    } else if let Ok(v) = value.extract::<Vec<SINumber>>() {
        let mut unit = SIUnit::DIMENSIONLESS;
        let (value, units): (Vec<_>, Vec<_>) = v.into_iter().map(|v| (v.value, v.unit)).unzip();
        for u in units {
            if unit != SIUnit::DIMENSIONLESS && unit != u {
                Err(QuantityError::InconsistentUnits {
                    unit1: unit,
                    unit2: u,
                })?;
            } else {
                unit = u
            }
        }
        let value: Array1<_> = Array1::from_vec(value);
        let value = value.into_pyarray(py).into_any().unbind();
        Bound::new(py, PySIObject::new(value, unit))
    } else {
        Ok(value.downcast_into::<PySIObject>()?)
    }
}

#[pyfunction]
fn linspace(
    py: Python,
    start: SINumber,
    end: SINumber,
    n: usize,
) -> Result<PySIObject, QuantityError> {
    if start.unit == end.unit {
        let value = Array1::linspace(start.value, end.value, n);
        let value = value.into_pyarray(py).into_any().unbind();
        Ok(PySIObject::new(value, start.unit))
    } else {
        Err(QuantityError::InconsistentUnits {
            unit1: start.unit,
            unit2: end.unit,
        })
    }
}

#[pyfunction]
fn logspace(
    py: Python,
    start: SINumber,
    end: SINumber,
    n: usize,
) -> Result<PySIObject, QuantityError> {
    if start.unit == end.unit {
        let value = Array1::logspace(10.0, start.value.log10(), end.value.log10(), n);
        let value = value.into_pyarray(py).into_any().unbind();
        Ok(PySIObject::new(value, start.unit))
    } else {
        Err(QuantityError::InconsistentUnits {
            unit1: start.unit,
            unit2: end.unit,
        })
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
pub fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<PySIObject>()?;
    m.add_class::<Angle>()?;
    m.add_function(wrap_pyfunction!(array1, m)?)?;
    m.add_function(wrap_pyfunction!(linspace, m)?)?;
    m.add_function(wrap_pyfunction!(logspace, m)?)?;

    add_constant(m, "SECOND", 1.0, _SECOND)?;
    add_constant(m, "METER", 1.0, _METER)?;
    add_constant(m, "KILOGRAM", 1.0, _KILOGRAM)?;
    add_constant(m, "AMPERE", 1.0, _AMPERE)?;
    add_constant(m, "KELVIN", 1.0, _KELVIN)?;
    add_constant(m, "MOL", 1.0, _MOL)?;
    add_constant(m, "CANDELA", 1.0, _CANDELA)?;

    add_constant(m, "DVCS", 9192631770.0, _HERTZ)?;
    add_constant(m, "CLIGHT", 299792458.0, _METER_PER_SECOND)?;
    add_constant(m, "PLANCK", 6.62607015e-34, _JOULE_SECOND)?;
    add_constant(m, "QE", 1.602176634e-19, _AMPERE_SECOND)?;
    add_constant(m, "KB", 1.380649e-23, _JOULE_PER_KELVIN)?;
    add_constant(m, "NAV", 6.02214076e23, _PER_MOL)?;
    add_constant(m, "KCD", 683.0, _LUMEN_PER_WATT)?;

    add_constant(m, "HERTZ", 1.0, _HERTZ)?;
    add_constant(m, "NEWTON", 1.0, _NEWTON)?;
    add_constant(m, "PASCAL", 1.0, _PASCAL)?;
    add_constant(m, "JOULE", 1.0, _JOULE)?;
    add_constant(m, "WATT", 1.0, _WATT)?;
    add_constant(m, "COULOMB", 1.0, _AMPERE_SECOND)?;
    add_constant(m, "VOLT", 1.0, _VOLT)?;
    add_constant(m, "FARAD", 1.0, _FARAD)?;
    add_constant(m, "OHM", 1.0, _OHM)?;
    add_constant(m, "SIEMENS", 1.0, _SIEMENS)?;
    add_constant(m, "WEBER", 1.0, _WEBER)?;
    add_constant(m, "TESLA", 1.0, _TESLA)?;
    add_constant(m, "HENRY", 1.0, _HENRY)?;

    add_constant(m, "ANGSTROM", 1e-10, _METER)?;
    add_constant(m, "AMU", 1.6605390671738466e-27, _KILOGRAM)?;
    add_constant(m, "AU", 149597870700.0, _METER)?;
    add_constant(m, "BAR", 1e5, _PASCAL)?;
    add_constant(m, "CALORIE", 4.184, _JOULE)?;
    m.add("CELSIUS", Celsius)?;
    add_constant(m, "DAY", 86400.0, _SECOND)?;
    m.add("DEBYE", Debye(1.0))?;
    m.add("DEGREES", Angle(1.0_f64.to_radians()))?;
    add_constant(m, "GRAM", 1e-3, _KILOGRAM)?;
    add_constant(m, "HOUR", 3600.0, _SECOND)?;
    add_constant(m, "LITER", 1e-3, _CUBIC_METER)?;
    add_constant(m, "MINUTE", 60.0, _SECOND)?;
    m.add("RADIANS", Angle(1.0))?;

    add_constant(m, "G", 6.6743e-11, SIUnit([3, -1, -2, 0, 0, 0, 0]))?;
    let rgas = 1.380649e-23 * 6.02214076e23;
    add_constant(m, "RGAS", rgas, _JOULE_PER_MOL_AND_KELVIN)?;

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

fn add_constant(m: &Bound<'_, PyModule>, name: &str, value: f64, unit: SIUnit) -> PyResult<()> {
    let value = PyFloat::new(m.py(), value).into_any().unbind();
    m.add(name, PySIObject::new(value, unit))
}
