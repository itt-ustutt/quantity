use crate::QuantityError;

use super::{SINumber, ANGSTROM, JOULE, _KELVIN};
use ang::Angle;
use pyo3::{exceptions::PyTypeError, prelude::*};

#[pyclass(module = "si_units")]
#[derive(Clone, Copy)]
pub struct Celsius;

#[pymethods]
impl Celsius {
    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            Ok(Bound::new(lhs.py(), SINumber::new(l + 273.15, _KELVIN))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray1(l.to_owned_array() * CELSIUS))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray2(l.to_owned_array() * CELSIUS))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray3(l.to_owned_array() * CELSIUS))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray4(l.to_owned_array() * CELSIUS))?.into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
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

    fn __pow__(&self, n: i32, _mod: Option<u32>) -> Result<SINumber, QuantityError> {
        if n % 2 == 1 {
            Err(QuantityError::DebyePower)
        } else {
            let value = (self.0.powi(2) * 1e-19 * ANGSTROM.value.powi(3)).powi(n / 2);
            let unit = (JOULE.unit * ANGSTROM.unit.powi(3)).powi(n / 2);
            Ok(SINumber::new(value, unit))
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
