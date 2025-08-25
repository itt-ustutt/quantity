use super::{Angle, Quantity, SIUnit};
use crate::fmt::PrintUnit;
#[cfg(feature = "nalgebra")]
use nalgebra::{DMatrix, DVector, Dyn};
#[cfg(feature = "ndarray")]
use ndarray::{Array, Dimension};
#[cfg(feature = "ndarray")]
use numpy::IntoPyArray;
#[cfg(any(feature = "nalgebra", feature = "ndarray"))]
use numpy::PyReadonlyArray;
#[cfg(feature = "nalgebra")]
use numpy::{PyReadonlyArray1, PyReadonlyArray2, ToPyArray};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{marker::PhantomData, sync::LazyLock};
use typenum::Integer;

static SIOBJECT: LazyLock<PyObject> = LazyLock::new(|| {
    Python::with_gil(|py| {
        PyModule::import(py, "si_units")
            .unwrap()
            .getattr("SIObject")
            .unwrap()
            .unbind()
    })
});

impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > IntoPyObject<'py> for Quantity<f64, SIUnit<T, L, M, I, THETA, N, J>>
{
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        SIOBJECT.bind(py).call1((self.0, unit))
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
    > IntoPyObject<'py> for Quantity<Array<f64, D>, SIUnit<T, L, M, I, THETA, N, J>>
{
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        let value = self.0.into_pyarray(py).into_any();
        SIOBJECT.bind(py).call1((value, unit))
    }
}

#[cfg(feature = "nalgebra")]
impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > IntoPyObject<'py> for Quantity<DMatrix<f64>, SIUnit<T, L, M, I, THETA, N, J>>
{
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        let value = self.0.to_pyarray(py).into_any();
        SIOBJECT.bind(py).call1((value, unit))
    }
}

#[cfg(feature = "nalgebra")]
impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > IntoPyObject<'py> for Quantity<DVector<f64>, SIUnit<T, L, M, I, THETA, N, J>>
{
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        let value = numpy::PyArray1::from_slice(py, self.0.data.as_vec()).into_any();
        SIOBJECT.bind(py).call1((value, unit))
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

#[cfg(feature = "nalgebra")]
impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > FromPyObject<'py> for Quantity<DVector<f64>, SIUnit<T, L, M, I, THETA, N, J>>
where
    Self: PrintUnit,
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let Ok((value, unit_from)) = ob.call_method0("__getnewargs__").and_then(|raw| {
            raw.extract::<(PyReadonlyArray1<f64>, [i8; 7])>()
                .map(|(m, u)| {
                    let m: nalgebra::DVectorView<f64, Dyn, Dyn> = m.try_as_matrix().unwrap();
                    (m.clone_owned(), u)
                })
        }) else {
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

#[cfg(feature = "nalgebra")]
impl<
        'py,
        T: Integer,
        L: Integer,
        M: Integer,
        I: Integer,
        THETA: Integer,
        N: Integer,
        J: Integer,
    > FromPyObject<'py> for Quantity<DMatrix<f64>, SIUnit<T, L, M, I, THETA, N, J>>
where
    Self: PrintUnit,
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let Ok((value, unit_from)) = ob.call_method0("__getnewargs__").and_then(|raw| {
            raw.extract::<(PyReadonlyArray2<f64>, [i8; 7])>()
                .map(|(m, u)| {
                    let m: nalgebra::DMatrixView<f64, Dyn, Dyn> = m.try_as_matrix().unwrap();
                    (m.clone_owned(), u)
                })
        }) else {
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

static ANGLE: LazyLock<PyObject> = LazyLock::new(|| {
    Python::with_gil(|py| {
        PyModule::import(py, "si_units")
            .unwrap()
            .getattr("Angle")
            .unwrap()
            .unbind()
    })
});

impl<'py> IntoPyObject<'py> for Angle {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        ANGLE.bind(py).call1((self.0,))
    }
}

impl<'py> FromPyObject<'py> for Angle {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let Ok(value) = ob
            .call_method0("__getnewargs__")
            .and_then(|raw| raw.extract::<f64>())
        else {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Missing units! Expected angle, got {}.",
                ob.call_method0("__repr__")?
            )));
        };
        Ok(Quantity(value, PhantomData))
    }
}
