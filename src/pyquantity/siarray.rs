use crate::si::*;
use crate::QuantityError;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4, ToPyArray};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::{PyNumberProtocol, PySequenceProtocol};

use super::PySINumber;

/// An one dimensional array of SI numbers.
#[pyclass(module = "eos_python.si_numbers", name = "SIArray1")]
#[derive(Clone)]
pub struct PySIArray1 {
    pub _data: SIArray1,
}

#[pyclass(module = "eos_python.si_numbers", name = "SIArray2")]
#[derive(Clone)]
pub struct PySIArray2 {
    pub _data: SIArray2,
}

#[pyclass(module = "eos_python.si_numbers", name = "SIArray3")]
#[derive(Clone)]
pub struct PySIArray3 {
    pub _data: SIArray3,
}

#[pyclass(module = "eos_python.si_numbers", name = "SIArray4")]
#[derive(Clone)]
pub struct PySIArray4 {
    pub _data: SIArray4,
}

// Adding the macro is not sufficient.
// You need to add _mul_ and _div_ in sinumber.rs to construct from numpy arrays.
impl_array!(PySIArray1, SIArray1, PyReadonlyArray1<f64>);
impl_array!(PySIArray2, SIArray2, PyReadonlyArray2<f64>);
impl_array!(PySIArray3, SIArray3, PyReadonlyArray3<f64>);
impl_array!(PySIArray4, SIArray4, PyReadonlyArray4<f64>);

#[pymethods]
impl PySIArray1 {
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
    #[text_signature = "(start, end, n)"]
    fn linspace(start: PySINumber, end: PySINumber, n: usize) -> Result<Self, QuantityError> {
        Ok(SIArray1::linspace(start._data, end._data, n)?.into())
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
    #[text_signature = "(start, end, n)"]
    fn logspace(start: PySINumber, end: PySINumber, n: usize) -> Result<Self, QuantityError> {
        Ok(SIArray1::logspace(start._data, end._data, n)?.into())
    }
}
