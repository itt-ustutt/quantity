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

    fn __add__(&self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            rhs.extract::<PyAngle>()
                .map(|r| Ok(PyCell::new(py, Self(self.0 + r.0))?.to_object(py)))?
        })
    }

    fn __sub__(&self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            rhs.extract::<PyAngle>()
                .map(|r| Ok(PyCell::new(py, Self(self.0 - r.0))?.to_object(py)))?
        })
    }

    fn __mul__(&self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            rhs.extract::<f64>()
                .map(|r| Ok(PyCell::new(py, Self(self.0 * r))?.to_object(py)))?
        })
    }

    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            lhs.extract::<f64>()
                .map(|l| Ok(PyCell::new(py, Self(l * self.0))?.to_object(py)))?
        })
    }

    fn __truediv__(&self, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            rhs.extract::<f64>()
                .map(|r| Ok(PyCell::new(py, Self(self.0 / r))?.to_object(py)))?
        })
    }

    fn __rtruediv__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            lhs.extract::<f64>()
                .map(|l| Ok(PyCell::new(py, Self(l / self.0))?.to_object(py)))?
        })
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok(Self(-self.0))
    }
}
