use ang::Angle;
use pyo3::prelude::*;

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
