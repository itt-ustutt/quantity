use crate::si::*;
use crate::QuantityError;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4, ToPyArray};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::{PyNumberProtocol, PySequenceProtocol};
use std::ops::Deref;

use super::PySINumber;

#[pyclass(name = "SIArray1")]
#[derive(Clone)]
pub struct PySIArray1(pub(crate) SIArray1);

#[pyclass(name = "SIArray2")]
#[derive(Clone)]
pub struct PySIArray2(pub(crate) SIArray2);

#[pyclass(name = "SIArray3")]
#[derive(Clone)]
pub struct PySIArray3(pub(crate) SIArray3);

#[pyclass(name = "SIArray4")]
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
    #[pyo3(text_signature = "(start, end, n)")]
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
    #[pyo3(text_signature = "(start, end, n)")]
    fn logspace(start: PySINumber, end: PySINumber, n: usize) -> Result<Self, QuantityError> {
        Ok(SIArray1::logspace(start.0, end.0, n)?.into())
    }
}
