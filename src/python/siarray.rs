use super::PyCelsius;
use crate::si::*;
use crate::{QuantityError, Unit};
use bincode::{deserialize, serialize};
use ndarray::{arr1, Array};
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::ops::Deref;

use super::PySINumber;

/// Create a new SIArray1
///
/// Parameters
/// ----------
/// value: {SINumber, [SINumber], SIArray1}
///     An SIArray1 or a scalar or list of SINumbers
///     to be converted to an SIArray1.
///     
#[pyclass(name = "SIArray1", module = "si_units")]
#[derive(Clone)]
pub struct PySIArray1(pub(crate) SIArray1);

#[pyclass(name = "SIArray2", module = "si_units")]
#[derive(Clone)]
pub struct PySIArray2(pub(crate) SIArray2);

#[pyclass(name = "SIArray3", module = "si_units")]
#[derive(Clone)]
pub struct PySIArray3(pub(crate) SIArray3);

#[pyclass(name = "SIArray4", module = "si_units")]
#[derive(Clone)]
pub struct PySIArray4(pub(crate) SIArray4);

// Adding the macro is not sufficient.
// You need to add _mul_ and _div_ in sinumber.rs to construct from numpy arrays.
impl_array!(PySIArray1, SIArray1, PyReadonlyArray1<f64>);
impl_array!(PySIArray2, SIArray2, PyReadonlyArray2<f64>);
impl_array!(PySIArray3, SIArray3, PyReadonlyArray3<f64>);
impl_array!(PySIArray4, SIArray4, PyReadonlyArray4<f64>);

#[pymethods]
impl PySIArray1 {
    #[new]
    #[pyo3(signature = (value=Python::with_gil(|py| PySINumber::new().into_py(py))))]
    fn new(py: Python, value: Py<PyAny>) -> PyResult<Self> {
        if let Ok(v) = value.extract::<PySINumber>(py) {
            return Ok(Self(arr1(&[1.0]) * v.0));
        };
        if let Ok(v) = value.extract::<Vec<PySINumber>>(py) {
            let v: Vec<_> = v.iter().map(|v| v.0).collect();
            return Ok(Self(SIArray1::from_vec(v)));
        };
        if let Ok(v) = value.extract::<Self>(py) {
            return Ok(v);
        };
        Err(PyErr::new::<PyTypeError, _>("not implemented!".to_string()))
    }

    /// Create a linearly spaced SIArray.
    ///
    /// Parameters
    /// ----------
    /// start: SINumber
    ///     The lowest value of the Array.
    /// end: SINumber
    ///     The highest value of the Array.
    /// n: int
    ///     The number of points.
    ///
    /// Returns
    /// -------
    /// SIArray1
    ///
    #[staticmethod]
    fn linspace(start: PySINumber, end: PySINumber, n: usize) -> Result<Self, QuantityError> {
        Ok(SIArray1::linspace(start.0, end.0, n)?.into())
    }

    /// Create a logarithmically spaced SIArray.
    ///
    /// Parameters
    /// ----------
    /// start: SINumber
    ///     The lowest value of the Array.
    /// end: SINumber
    ///     The highest value of the Array.
    /// n: int
    ///     The number of points.
    ///
    /// Returns
    /// -------
    /// SIArray1
    ///
    #[staticmethod]
    fn logspace(start: PySINumber, end: PySINumber, n: usize) -> Result<Self, QuantityError> {
        Ok(SIArray1::logspace(start.0, end.0, n)?.into())
    }
}

#[pymethods]
impl PySIArray2 {
    #[new]
    fn new() -> Self {
        Self(SIArray2 {
            value: Array::zeros([0; 2]),
            unit: SIUnit::DIMENSIONLESS,
        })
    }
}

#[pymethods]
impl PySIArray3 {
    #[new]
    fn new() -> Self {
        Self(SIArray3 {
            value: Array::zeros([0; 3]),
            unit: SIUnit::DIMENSIONLESS,
        })
    }
}

#[pymethods]
impl PySIArray4 {
    #[new]
    fn new() -> Self {
        Self(SIArray4 {
            value: Array::zeros([0; 4]),
            unit: SIUnit::DIMENSIONLESS,
        })
    }
}

#[pymethods]
impl PySIArray1 {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<PySINumber> {
        if idx < 0 || idx >= self.0.len() as isize {
            Err(PyIndexError::new_err("array index out of bounds"))
        } else {
            Ok(PySINumber(self.0.get(idx as usize)))
        }
    }

    fn __setitem__(&mut self, idx: isize, value: PySINumber) -> PyResult<()> {
        if idx < 0 || idx >= self.0.len() as isize {
            Err(PyIndexError::new_err("array index out of bounds"))
        } else {
            Ok(self.0.try_set(idx as usize, value.0)?)
        }
    }
}

#[pymethods]
impl PySIArray2 {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }
}

#[pymethods]
impl PySIArray3 {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }
}

#[pymethods]
impl PySIArray4 {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }
}
