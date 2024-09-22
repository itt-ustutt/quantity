use crate::{PySIObject, QuantityError, _JOULE, _KELVIN, _METER};
use ang::Angle;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyNotImplemented;
use pyo3::PyTypeInfo;

#[pyclass(module = "si_units")]
#[derive(Clone, Copy)]
pub struct Celsius;

#[pymethods]
impl Celsius {
    fn __rmul__(&self, lhs: &Bound<'_, PyAny>) -> PyResult<PySIObject> {
        let delta: Py<PyAny> = 273.15.into_py(lhs.py());
        let delta = delta.bind(lhs.py());
        let mut value = lhs.call_method1("__add__", (delta,))?;
        if PyNotImplemented::is_exact_type_of_bound(&value) {
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            return Ok(Bound::new(lhs.py(), Debye(l * self.0))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __pow__(&self, py: Python, n: i32, _mod: Option<u32>) -> PyResult<PySIObject> {
        if n % 2 == 1 {
            Err(QuantityError::DebyePower)?
        } else {
            let value = (self.0.powi(2) * 1e-19 * 1e-30).powi(n / 2);
            let unit = (_JOULE * _METER.powi(3)).powi(n / 2);
            Ok(PySIObject::new(value.into_py(py), unit))
        }
    }
}

#[pyclass(name = "Angle", module = "si_units")]
#[derive(Clone, Copy)]
pub struct PyAngle(pub(crate) Angle<f64>);

impl From<Angle> for PyAngle {
    fn from(angle: Angle) -> Self {
        Self(angle)
    }
}

impl From<PyAngle> for Angle {
    fn from(angle: PyAngle) -> Self {
        angle.0
    }
}

#[pymethods]
impl PyAngle {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
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

    fn __truediv__(&self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }
}
