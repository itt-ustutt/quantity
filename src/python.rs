use super::{Quantity, SIUnit};
use crate::fmt::PrintUnit;
#[cfg(feature = "ndarray")]
use ndarray::{Array, Dimension};
#[cfg(feature = "ndarray")]
use numpy::{IntoPyArray, PyReadonlyArray};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{marker::PhantomData, sync::LazyLock};
use typenum::Integer;

static SIOBJECT: LazyLock<PyObject> = LazyLock::new(|| {
    Python::with_gil(|py| {
        PyModule::import_bound(py, "si_units")
            .unwrap()
            .getattr("SIObject")
            .unwrap()
            .unbind()
    })
});

impl<T: Integer, L: Integer, M: Integer, I: Integer, THETA: Integer, N: Integer, J: Integer>
    IntoPy<PyObject> for Quantity<f64, SIUnit<T, L, M, I, THETA, N, J>>
{
    fn into_py(self, py: Python<'_>) -> PyObject {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        SIOBJECT.call1(py, (self.0, unit)).unwrap()
    }
}

#[cfg(feature = "ndarray")]
impl<
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
        D: Dimension,
    > IntoPy<PyObject> for Quantity<Array<f64, D>, SIUnit<T, L, M, I, THETA, N, J>>
{
    fn into_py(self, py: Python<'_>) -> PyObject {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        let value = self.0.into_pyarray_bound(py).into_any();
        SIOBJECT.call1(py, (value, unit)).unwrap()
    }
}

impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > FromPyObject<'py> for Quantity<f64, SIUnit<T, L, M, I, THETA, N, J>>
where
    Self: PrintUnit,
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let Ok((value, unit_from)) = ob
            .call_method0("__getnewargs__")
            .and_then(|raw| raw.extract::<(f64, [i8; 7])>())
        else {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Missing units! Expected {}, got {}.",
                Self::UNIT,
                ob.call_method0("__repr__")?
            )));
        };
        let unit_into = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        if unit_into == unit_from {
            Ok(Quantity(value, PhantomData))
        } else {
            Err(PyErr::new::<PyValueError, _>(format!(
                "Wrong units! Expected {}, got {}.",
                Self::UNIT,
                ob.call_method0("__repr__")?
            )))
        }
    }
}

#[cfg(feature = "ndarray")]
impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
        D: Dimension,
    > FromPyObject<'py> for Quantity<Array<f64, D>, SIUnit<T, L, M, I, THETA, N, J>>
where
    Self: PrintUnit,
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let Ok((value, unit_from)) = ob
            .call_method0("__getnewargs__")
            .and_then(|raw| raw.extract::<(PyReadonlyArray<f64, D>, [i8; 7])>())
        else {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Missing units! Expected {}, got {}.",
                Self::UNIT,
                ob.call_method0("__repr__")?
            )));
        };
        let value = value.as_array().to_owned();
        let unit_into = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        if unit_into == unit_from {
            Ok(Quantity(value, PhantomData))
        } else {
            Err(PyErr::new::<PyValueError, _>(format!(
                "Wrong units! Expected {}, got {}.",
                Self::UNIT,
                ob.call_method0("__repr__")?
            )))
        }
    }
}
