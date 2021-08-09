use crate::{si::*, QuantityError};
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyNumberProtocol;

use super::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

#[pyclass(name = "SINumber")]
#[derive(Clone)]
pub struct PySINumber(pub SINumber);

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
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __sub__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PySINumber>() {
                return Ok(PyCell::new(py, Self(lhs.0 - r.0))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __mul__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
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
                return Ok(PyCell::new(py, PySIArray1(lhs.0 * r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray2>() {
                return Ok(PyCell::new(py, PySIArray2(lhs.0 * r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray3>() {
                return Ok(PyCell::new(py, PySIArray3(lhs.0 * r.0))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySIArray4>() {
                return Ok(PyCell::new(py, PySIArray4(lhs.0 * r.0))?.to_object(py));
            };
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
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
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
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(self.0 * l))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    // // fn __imul__(&mut self, other: &PyAny) -> PyResult<()> {
    // //     if let Ok(n) = other.extract::<f64>() {
    // //         self.0 = self.0 * n;
    // //         return Ok(());
    // //     };
    // //     if let Ok(n) = other.extract::<PyRef<SINumber>>() {
    // //         self.0 = self.0 * n.0;
    // //         return Ok(());
    // //     };
    // //     Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    // // }

    fn __truediv__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(py, PySIArray1(lhs.0 * inv))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(py, PySIArray2(lhs.0 * inv))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(py, PySIArray3(lhs.0 * inv))?.to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(py, PySIArray4(lhs.0 * inv))?.to_object(py));
            };
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
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __rtruediv__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
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
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, Self(l / self.0))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }

    fn __pow__(lhs: PyRef<'p, Self>, rhs: i32, _mod: Option<u32>) -> Self {
        Self(lhs.0.powi(rhs))
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok(Self(-self.0))
    }
}

#[pyclass(name = "Celsius")]
#[derive(Clone)]
pub struct PyCelsius;

#[pyproto]
impl PyNumberProtocol for PyCelsius {
    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
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
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(py, PySINumber(l * CELSIUS))?.to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        })
    }
}
