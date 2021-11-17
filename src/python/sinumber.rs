use crate::{si::*, QuantityError};
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4, ToPyArray};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyNumberProtocol;

use super::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

#[pyclass(name = "SINumber")]
#[derive(Clone, Copy)]
pub struct PySINumber(pub(crate) SINumber);

impl From<SINumber> for PySINumber {
    fn from(si: SINumber) -> Self {
        Self(si)
    }
}

impl From<PySINumber> for SINumber {
    fn from(si: PySINumber) -> Self {
        si.0
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PySINumber {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        })
    }
}

#[pymethods]
impl PySINumber {
    /// Try to calculate the square root of self.
    ///
    /// Examples
    /// --------
    ///
    /// >>> import si
    /// >>> m2 = METER**2
    /// >>> m2.sqrt()
    /// 1  m
    pub fn sqrt(&self) -> Result<Self, QuantityError> {
        Ok(Self(self.0.sqrt()?))
    }

    /// Try to calculate the cubic root of self.
    ///
    /// Examples
    /// --------
    ///
    /// >>> import si
    /// >>> m3 = METER**3
    /// >>> m3.cbrt()
    /// 1  m
    pub fn cbrt(&self) -> Result<Self, QuantityError> {
        Ok(Self(self.0.cbrt()?))
    }

    /// Test if the quantity has the same unit as the argument.
    ///
    /// Parameters
    /// ----------
    /// other : SINumber
    ///     The unit that is compared.
    ///
    /// Returns
    /// -------
    /// bool
    #[pyo3(text_signature = "($self, other)")]
    fn has_unit(&self, other: PySINumber) -> bool {
        self.0.has_unit(&other.0)
    }

    #[classattr]
    fn __array_priority__() -> u64 {
        1000
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
    }
}

#[pyproto]
impl PyNumberProtocol for PySINumber {
    /// Addition
    fn __add__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PySINumber>() {
                return Ok(PyCell::new(py, Self(lhs.0 + r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray1>() {
                return Ok(PyCell::new(py, PySIArray1(lhs.0 + r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray2>() {
                return Ok(PyCell::new(py, PySIArray2(lhs.0 + r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray3>() {
                return Ok(PyCell::new(py, PySIArray3(lhs.0 + r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray4>() {
                return Ok(PyCell::new(py, PySIArray4(lhs.0 + r.0))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __sub__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PySINumber>() {
                return Ok(PyCell::new(py, Self(lhs.0 - r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray1>() {
                return Ok(PyCell::new(py, PySIArray1(lhs.0 - r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray2>() {
                return Ok(PyCell::new(py, PySIArray2(lhs.0 - r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray3>() {
                return Ok(PyCell::new(py, PySIArray3(lhs.0 - r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray4>() {
                return Ok(PyCell::new(py, PySIArray4(lhs.0 - r.0))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __mul__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(lhs.0 * r))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySINumber>() {
                let result = lhs.0 * r.0;
                return Ok(match result.into_value() {
                    Ok(r) => r.to_object(py),
                    Err(_) => PyCell::new(py, Self(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(py, PySIArray1(lhs.0 * r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(py, PySIArray2(lhs.0 * r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(py, PySIArray3(lhs.0 * r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(py, PySIArray4(lhs.0 * r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray1>() {
                let result = lhs.0 * r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray1(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray2>() {
                let result = lhs.0 * r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray2(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray3>() {
                let result = lhs.0 * r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray3(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray4>() {
                let result = lhs.0 * r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray4(result))?.to_object(py),
                });
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(self.0 * l))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(py, PySIArray1(self.0 * l.to_owned_array()))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(py, PySIArray2(self.0 * l.to_owned_array()))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(py, PySIArray3(self.0 * l.to_owned_array()))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(py, PySIArray4(self.0 * l.to_owned_array()))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __truediv__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(lhs.0 / r))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySINumber>() {
                let result = lhs.0 / r.0;
                return Ok(match result.into_value() {
                    Ok(r) => r.to_object(py),
                    Err(_) => PyCell::new(py, Self(result))?.to_object(py),
                });
            };
            if let Ok(_) = rhs.extract::<PyCelsius>() {
                return Ok((lhs.0 / CELSIUS).to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(py, PySIArray1(lhs.0 / r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(py, PySIArray2(lhs.0 / r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(py, PySIArray3(lhs.0 / r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(py, PySIArray4(lhs.0 / r.to_owned_array()))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray1>() {
                let result = lhs.0 / r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray1(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray2>() {
                let result = lhs.0 / r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray2(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray3>() {
                let result = lhs.0 / r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray3(result))?.to_object(py),
                });
            };
            if let Ok(r) = rhs.extract::<PySIArray4>() {
                let result = lhs.0 / r.0;
                return Ok(match result.value() {
                    Ok(r) => r.to_pyarray(py).to_object(py),
                    Err(_) => PyCell::new(py, PySIArray4(result))?.to_object(py),
                });
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __rtruediv__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(l / self.0))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(
                    PyCell::new(py, PySIArray1(1.0 / self.0 * l.to_owned_array()))?.to_object(py),
                );
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(
                    PyCell::new(py, PySIArray2(1.0 / self.0 * l.to_owned_array()))?.to_object(py),
                );
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(
                    PyCell::new(py, PySIArray3(1.0 / self.0 * l.to_owned_array()))?.to_object(py),
                );
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(
                    PyCell::new(py, PySIArray4(1.0 / self.0 * l.to_owned_array()))?.to_object(py),
                );
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __pow__(lhs: PyRef<'p, Self>, rhs: i32, _mod: Option<u32>) -> Self {
        Self(lhs.0.powi(rhs))
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }
}

#[pyclass(name = "Celsius")]
#[derive(Clone, Copy)]
pub struct PyCelsius;

#[pyproto]
impl PyNumberProtocol for PyCelsius {
    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, PySINumber(l * CELSIUS))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(py, PySIArray1(l.to_owned_array() * CELSIUS))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(py, PySIArray2(l.to_owned_array() * CELSIUS))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(py, PySIArray3(l.to_owned_array() * CELSIUS))?.to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(py, PySIArray4(l.to_owned_array() * CELSIUS))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }
}

#[pyclass(name = "Debye")]
#[derive(Clone, Copy)]
pub struct PyDebye(pub(crate) Debye);

#[pymethods]
impl PyDebye {
    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PyDebye {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyproto]
impl PyNumberProtocol for PyDebye {
    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, PyDebye(l * self.0.clone()))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __pow__(lhs: PyRef<'p, Self>, rhs: i32, _mod: Option<u32>) -> PySINumber {
        PySINumber(lhs.0.powi(rhs))
    }
}
