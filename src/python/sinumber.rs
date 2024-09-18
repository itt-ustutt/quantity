use crate::*;
use bincode::{deserialize, serialize};
use numpy::prelude::*;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyFloat;

use super::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

#[pyclass(name = "SINumber", module = "si_units")]
#[derive(Clone, Copy, Default)]
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

#[pymethods]
impl PySINumber {
    #[new]
    // Required for pickling SINumbers
    pub fn new() -> Self {
        Self::default()
    }

    fn __setstate__(&mut self, state: &Bound<'_, PyBytes>) {
        self.0 = deserialize(state.as_bytes()).unwrap();
    }

    fn __getstate__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &serialize(&self.0).unwrap())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
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
    fn has_unit(&self, other: PySINumber) -> bool {
        self.0.has_unit(&other.0)
    }

    #[classattr]
    fn __array_priority__() -> u64 {
        1000
    }

    fn __add__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<PySINumber>() {
            return Ok(Bound::new(rhs.py(), Self(self.0 + r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray1>() {
            return Ok(Bound::new(rhs.py(), PySIArray1(self.0 + r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray2>() {
            return Ok(Bound::new(rhs.py(), PySIArray2(self.0 + r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray3>() {
            return Ok(Bound::new(rhs.py(), PySIArray3(self.0 + r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray4>() {
            return Ok(Bound::new(rhs.py(), PySIArray4(self.0 + r.0))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __sub__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<PySINumber>() {
            return Ok(Bound::new(rhs.py(), Self(self.0 - r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray1>() {
            return Ok(Bound::new(rhs.py(), PySIArray1(self.0 - r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray2>() {
            return Ok(Bound::new(rhs.py(), PySIArray2(self.0 - r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray3>() {
            return Ok(Bound::new(rhs.py(), PySIArray3(self.0 - r.0))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray4>() {
            return Ok(Bound::new(rhs.py(), PySIArray4(self.0 - r.0))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __mul__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<f64>() {
            return Ok(Bound::new(rhs.py(), Self(self.0 * r))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySINumber>() {
            let result = self.0 * r.0;
            return Ok(match result.into_value() {
                Ok(r) => PyFloat::new_bound(rhs.py(), r).into_any(),
                Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray1(self.0 * r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray2(self.0 * r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray3(self.0 * r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray4(self.0 * r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray1>() {
            let result = self.0 * r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray1(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray2>() {
            let result = self.0 * r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray2(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray3>() {
            let result = self.0 * r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray3(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray4>() {
            let result = self.0 * r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray4(result))?.into_any(),
            });
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            return Ok(Bound::new(lhs.py(), Self(self.0 * l))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray1(self.0 * l.to_owned_array()))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray2(self.0 * l.to_owned_array()))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray3(self.0 * l.to_owned_array()))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray4(self.0 * l.to_owned_array()))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __truediv__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<f64>() {
            return Ok(Bound::new(rhs.py(), Self(self.0 / r))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySINumber>() {
            let result = self.0 / r.0;
            return Ok(match result.into_value() {
                Ok(r) => PyFloat::new_bound(rhs.py(), r).into_any(),
                Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
            });
        };
        if rhs.extract::<PyCelsius>().is_ok() {
            return Ok(PyFloat::new_bound(rhs.py(), self.0 / CELSIUS).into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray1(self.0 / r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray2(self.0 / r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray3(self.0 / r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
            return Ok(Bound::new(rhs.py(), PySIArray4(self.0 / r.to_owned_array()))?.into_any());
        };
        if let Ok(r) = rhs.extract::<PySIArray1>() {
            let result = self.0 / r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray1(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray2>() {
            let result = self.0 / r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray2(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray3>() {
            let result = self.0 / r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray3(result))?.into_any(),
            });
        };
        if let Ok(r) = rhs.extract::<PySIArray4>() {
            let result = self.0 / r.0;
            return Ok(match result.value() {
                Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                Err(_) => Bound::new(rhs.py(), PySIArray4(result))?.into_any(),
            });
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __rtruediv__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            return Ok(Bound::new(lhs.py(), Self(l / self.0))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
            return Ok(
                Bound::new(lhs.py(), PySIArray1(1.0 / self.0 * l.to_owned_array()))?.into_any(),
            );
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
            return Ok(
                Bound::new(lhs.py(), PySIArray2(1.0 / self.0 * l.to_owned_array()))?.into_any(),
            );
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
            return Ok(
                Bound::new(lhs.py(), PySIArray3(1.0 / self.0 * l.to_owned_array()))?.into_any(),
            );
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
            return Ok(
                Bound::new(lhs.py(), PySIArray4(1.0 / self.0 * l.to_owned_array()))?.into_any(),
            );
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __pow__(&self, rhs: i32, _mod: Option<u32>) -> Self {
        Self(self.0.powi(rhs))
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }
}

#[pyclass(name = "Celsius", module = "si_units")]
#[derive(Clone, Copy)]
pub struct PyCelsius;

#[pymethods]
impl PyCelsius {
    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            return Ok(Bound::new(lhs.py(), PySINumber(l * CELSIUS))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray1(l.to_owned_array() * CELSIUS))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray2(l.to_owned_array() * CELSIUS))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray3(l.to_owned_array() * CELSIUS))?.into_any());
        };
        if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
            return Ok(Bound::new(lhs.py(), PySIArray4(l.to_owned_array() * CELSIUS))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }
}

#[pyclass(name = "Debye", module = "si_units")]
#[derive(Clone, Copy)]
pub struct PyDebye(pub(crate) Debye);

#[pymethods]
impl PyDebye {
    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            return Ok(Bound::new(lhs.py(), PyDebye(l * self.0))?.into_any());
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    fn __pow__(&self, rhs: i32, _mod: Option<u32>) -> PySINumber {
        PySINumber(self.0.powi(rhs))
    }
}
