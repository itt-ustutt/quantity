use crate::{try_add, try_sub, QuantityError, SIUnit};
use bincode::{deserialize, serialize};
// use numpy::prelude::*;
// use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4};
use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
// use super::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

#[pyclass(module = "si_units")]
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct SINumber {
    pub(crate) value: f64,
    pub(crate) unit: SIUnit,
}

impl SINumber {
    pub fn new(value: f64, unit: SIUnit) -> Self {
        Self { value, unit }
    }
}

#[pymethods]
impl SINumber {
    #[new]
    // Required for pickling SINumbers
    pub fn py_new() -> Self {
        Self::default()
    }

    fn __setstate__(&mut self, state: &Bound<'_, PyBytes>) {
        *self = deserialize(state.as_bytes()).unwrap();
    }

    fn __getstate__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &serialize(self).unwrap())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.to_latex())
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        if self.unit == other.unit {
            Ok(match op {
                CompareOp::Eq => self.value == other.value,
                CompareOp::Ne => self.value != other.value,
                CompareOp::Lt => self.value < other.value,
                CompareOp::Le => self.value <= other.value,
                CompareOp::Gt => self.value > other.value,
                CompareOp::Ge => self.value >= other.value,
            })
        } else {
            Err(PyErr::new::<PyValueError, _>("".to_string()))
        }
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
        Ok(SINumber::new(self.value.sqrt(), self.unit.sqrt()?))
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
        Ok(SINumber::new(self.value.cbrt(), self.unit.cbrt()?))
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
    pub fn has_unit(&self, other: SINumber) -> bool {
        self.unit.eq(&other.unit)
    }

    #[classattr]
    fn __array_priority__() -> u64 {
        1000
    }

    fn __add__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<SINumber>() {
            let (value, unit) = try_add(self.value, self.unit, r.value, r.unit)?;
            Ok(Bound::new(rhs.py(), Self::new(value, unit))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray1>() {
        //     Ok(Bound::new(rhs.py(), PySIArray1(self.0 + r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray2>() {
        //     Ok(Bound::new(rhs.py(), PySIArray2(self.0 + r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray3>() {
        //     Ok(Bound::new(rhs.py(), PySIArray3(self.0 + r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray4>() {
        //     Ok(Bound::new(rhs.py(), PySIArray4(self.0 + r.0))?.into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __sub__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<SINumber>() {
            let (value, unit) = try_sub(self.value, self.unit, r.value, r.unit)?;
            Ok(Bound::new(rhs.py(), Self::new(value, unit))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray1>() {
        //     Ok(Bound::new(rhs.py(), PySIArray1(self.0 - r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray2>() {
        //     Ok(Bound::new(rhs.py(), PySIArray2(self.0 - r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray3>() {
        //     Ok(Bound::new(rhs.py(), PySIArray3(self.0 - r.0))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray4>() {
        //     Ok(Bound::new(rhs.py(), PySIArray4(self.0 - r.0))?.into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __mul__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<f64>() {
            Ok(Bound::new(rhs.py(), Self::new(self.value * r, self.unit))?.into_any())
        } else if let Ok(r) = rhs.extract::<SINumber>() {
            let value = self.value * r.value;
            let unit = self.unit * r.unit;
            Ok(if unit.is_dimensionless() {
                value.into_py(rhs.py()).into_bound(rhs.py())
            } else {
                Bound::new(rhs.py(), Self::new(value, unit))?.into_any()
            })
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray1(self.0 * r.to_owned_array()))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray2(self.0 * r.to_owned_array()))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray3(self.0 * r.to_owned_array()))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray4(self.0 * r.to_owned_array()))?.into_any())
        // } else if let Ok(r) = rhs.extract::<PySIArray1>() {
        //     let result = self.0 * r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray1(result))?.into_any(),
        //     })
        // } else if let Ok(r) = rhs.extract::<PySIArray2>() {
        //     let result = self.0 * r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray2(result))?.into_any(),
        //     })
        // } else if let Ok(r) = rhs.extract::<PySIArray3>() {
        //     let result = self.0 * r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray3(result))?.into_any(),
        //     })
        // } else if let Ok(r) = rhs.extract::<PySIArray4>() {
        //     let result = self.0 * r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray4(result))?.into_any(),
        //     })
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __rmul__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            Ok(Bound::new(lhs.py(), Self::new(l * self.value, self.unit))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray1(self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray2(self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray3(self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray4(self.0 * l.to_owned_array()))?.into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __truediv__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(r) = rhs.extract::<f64>() {
            Ok(Bound::new(rhs.py(), Self::new(self.value / r, self.unit))?.into_any())
        } else if let Ok(r) = rhs.extract::<SINumber>() {
            let value = self.value / r.value;
            let unit = self.unit / r.unit;
            Ok(if unit.is_dimensionless() {
                value.into_py(rhs.py()).into_bound(rhs.py())
            } else {
                Bound::new(rhs.py(), Self::new(value, unit))?.into_any()
            })
        // } else if rhs.extract::<PyCelsius>().is_ok() {
        //     Ok(PyFloat::new_bound(rhs.py(), self.0 / CELSIUS).into_any());
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray1(self.0 / r.to_owned_array()))?.into_any());
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray2(self.0 / r.to_owned_array()))?.into_any());
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray3(self.0 / r.to_owned_array()))?.into_any());
        // } else if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
        //     Ok(Bound::new(rhs.py(), PySIArray4(self.0 / r.to_owned_array()))?.into_any());
        // } else if let Ok(r) = rhs.extract::<PySIArray1>() {
        //     let result = self.0 / r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray1(result))?.into_any(),
        //     });
        // } else if let Ok(r) = rhs.extract::<PySIArray2>() {
        //     let result = self.0 / r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray2(result))?.into_any(),
        //     });
        // } else if let Ok(r) = rhs.extract::<PySIArray3>() {
        //     let result = self.0 / r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray3(result))?.into_any(),
        //     });
        // } else if let Ok(r) = rhs.extract::<PySIArray4>() {
        //     let result = self.0 / r.0;
        //     Ok(match result.value() {
        //         Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
        //         Err(_) => Bound::new(rhs.py(), PySIArray4(result))?.into_any(),
        //     });
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __rtruediv__<'py>(&self, lhs: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(l) = lhs.extract::<f64>() {
            Ok(Bound::new(lhs.py(), Self::new(l / self.value, self.unit.recip()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray1(1.0 / self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray2(1.0 / self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray3(1.0 / self.0 * l.to_owned_array()))?.into_any())
        // } else if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
        //     Ok(Bound::new(lhs.py(), PySIArray4(1.0 / self.0 * l.to_owned_array()))?.into_any())
        } else {
            Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
        }
    }

    fn __pow__(&self, i: i32, _mod: Option<u32>) -> Self {
        Self {
            value: self.value.powi(i),
            unit: self.unit.powi(i),
        }
    }

    fn __neg__(&self) -> Self {
        Self {
            value: -self.value,
            unit: self.unit,
        }
    }

    fn __abs__(&self) -> Self {
        Self {
            value: self.value.abs(),
            unit: self.unit,
        }
    }
}
