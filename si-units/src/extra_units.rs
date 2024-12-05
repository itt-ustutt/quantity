use crate::{PySIObject, QuantityError, _JOULE, _KELVIN, _METER};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyNotImplemented};
use pyo3::PyTypeInfo;

#[pyclass(module = "si_units")]
#[derive(Clone, Copy)]
pub struct Celsius;

#[pymethods]
impl Celsius {
    fn __rmul__(&self, lhs: &Bound<'_, PyAny>) -> PyResult<PySIObject> {
        let delta = PyFloat::new(lhs.py(), 273.15);
        let mut value = lhs.call_method1("__add__", (&delta,))?;
        if PyNotImplemented::is_exact_type_of(&value) {
            value = delta.call_method1("__add__", (lhs,))?;
        }
        Ok(PySIObject::new(value.unbind(), _KELVIN))
    }

    #[classattr]
    fn __array_priority__() -> u64 {
        1000
    }
}

#[pyclass(module = "si_units")]
#[derive(Clone, Copy)]
pub struct Debye(pub f64);

#[pymethods]
impl Debye {
    fn _repr_latex_(&self) -> String {
        format!("${}$", self.to_latex())
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __rmul__(&self, lhs: f64) -> Self {
        Self(lhs * self.0)
    }

    fn __pow__(&self, py: Python, n: i32, _mod: Option<u32>) -> PyResult<PySIObject> {
        if n % 2 == 1 {
            Err(QuantityError::DebyePower)?
        } else {
            let value = PyFloat::new(py, (self.0.powi(2) * 1e-19 * 1e-30).powi(n / 2));
            let value = value.into_any().unbind();
            let unit = (_JOULE * _METER.powi(3)).powi(n / 2);
            Ok(PySIObject::new(value, unit))
        }
    }
}

#[pyclass(module = "si_units")]
#[derive(Clone, Copy)]
pub struct Angle(pub(crate) f64);

#[pymethods]
impl Angle {
    #[new]
    fn new(value: f64) -> Self {
        Self(value)
    }

    fn __getnewargs__(&self) -> f64 {
        self.0
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.to_latex())
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __add__(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn __sub__(&self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }

    fn __mul__(&self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }

    fn __rmul__(&self, lhs: f64) -> Self {
        Self(lhs * self.0)
    }

    fn __truediv__<'py>(&self, rhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<f64>() {
            Ok(Bound::new(rhs.py(), Self(self.0 / r))?.into_any())
        } else if let Ok(r) = rhs.extract::<Self>() {
            Ok(PyFloat::new(rhs.py(), self.0 / r.0).into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>(format!(
                "Can't divide angle by {}",
                rhs
            )))
        }
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn sin(&self) -> f64 {
        self.0.sin()
    }

    fn cos(&self) -> f64 {
        self.0.cos()
    }

    fn tan(&self) -> f64 {
        self.0.tan()
    }
}
