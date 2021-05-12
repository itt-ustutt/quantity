use crate::{si::*, QuantityError};
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyNumberProtocol;

use super::{PySIArray1, PySIArray2, PySIArray3, PySIArray4};

/// A scalar value with SI unit.
///
/// .. note::
///
///     A scalar value is constructed by multiplying or dividing
///     a float with a "unit". See the examples below.
///
/// Examples
/// --------
///
/// Generating a SINumber is done by multiplication with a
/// "unit", i.e. a SINumber defined in the library.
///
/// >>> from eos_python.si_number import BAR
/// >>> pressure = 1.0 * BAR
/// >>> pressure
/// 100 kPa
///
/// When a SINumber is dimensionless after an arithmetic operation,
/// the return value is a float. This way, we can convert units as well.
///
/// >>> from eos_python.si_number import BAR, MILLI, PASCAL
/// >>> pressure = 1.0 * BAR
/// >>> pressure / (MILLI * PASCAL)
/// 100000000.0
///
/// Some interaction with numpy.ndarrays is also provided. For example
/// we can create a SIArray1 using multiplication equivalent to the example above
///
/// >>> from eos_python.si_number import BAR
/// >>> import numpy as np
/// >>> pressures = np.array([1.0, 2.0, 3.0, 5.0]) * BAR
/// >>> pressures
/// [100000, 200000, 300000, 500000] Pa
#[pyclass(module = "eos_python.si_numbers", name = "SINumber")]
#[derive(Clone)]
pub struct PySINumber {
    pub _data: SINumber,
}

impl From<SINumber> for PySINumber {
    fn from(si: SINumber) -> Self {
        Self { _data: si }
    }
}

impl From<PySINumber> for SINumber {
    fn from(si: PySINumber) -> Self {
        si._data
    }
}

#[pymethods]
impl PySINumber {
    #[classattr]
    fn __array_priority__() -> u64 {
        1000
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
    #[text_signature = "($self, other)"]
    fn has_unit(&self, other: PySINumber) -> bool {
        self._data.has_unit(&other._data)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PySINumber {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self._data.to_string())
    }
}

#[pymethods]
impl PySINumber {
    fn _repr_latex_(&self) -> String {
        self._data.to_latex()
    }
}

#[pyproto]
impl PyNumberProtocol for PySINumber {
    /// Addition
    fn __add__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PySINumber>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: lhs._data + r._data,
                    },
                )?
                .to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    fn __sub__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PySINumber>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: lhs._data - r._data,
                    },
                )?
                .to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    fn __mul__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray1 {
                        _data: lhs._data * r.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray2 {
                        _data: lhs._data * r.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray3 {
                        _data: lhs._data * r.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray4 {
                        _data: lhs._data * r.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<f64>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: lhs._data * r,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySINumber>() {
                let result = lhs._data * r._data;
                return Ok(match result.into_value() {
                    Ok(r) => r.to_object(py),
                    Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                });
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray1 {
                        _data: self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray2 {
                        _data: self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray3 {
                        _data: self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray4 {
                        _data: self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: self._data * l,
                    },
                )?
                .to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    // // fn __imul__(&mut self, other: &PyAny) -> PyResult<()> {
    // //     if let Ok(n) = other.extract::<f64>() {
    // //         self._data = self._data * n;
    // //         return Ok(());
    // //     };
    // //     if let Ok(n) = other.extract::<PyRef<SINumber>>() {
    // //         self._data = self._data * n._data;
    // //         return Ok(());
    // //     };
    // //     Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
    // // }

    fn __truediv__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(r) = rhs.extract::<PyReadonlyArray1<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(
                    py,
                    PySIArray1 {
                        _data: lhs._data * inv,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray2<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(
                    py,
                    PySIArray2 {
                        _data: lhs._data * inv,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray3<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(
                    py,
                    PySIArray3 {
                        _data: lhs._data * inv,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PyReadonlyArray4<f64>>() {
                let inv = 1.0f64 / r.to_owned_array();
                return Ok(PyCell::new(
                    py,
                    PySIArray4 {
                        _data: lhs._data * inv,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<f64>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: lhs._data / r,
                    },
                )?
                .to_object(py));
            };
            if let Ok(r) = rhs.extract::<PySINumber>() {
                let result = lhs._data / r._data;
                return Ok(match result.into_value() {
                    Ok(r) => r.to_object(py),
                    Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                });
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    fn __rtruediv__(&self, lhs: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(l) = lhs.extract::<PyReadonlyArray1<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray1 {
                        _data: 1.0 / self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray2<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray2 {
                        _data: 1.0 / self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray3<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray3 {
                        _data: 1.0 / self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<PyReadonlyArray4<f64>>() {
                return Ok(PyCell::new(
                    py,
                    PySIArray4 {
                        _data: 1.0 / self._data * l.to_owned_array(),
                    },
                )?
                .to_object(py));
            };
            if let Ok(l) = lhs.extract::<f64>() {
                return Ok(PyCell::new(
                    py,
                    Self {
                        _data: l / self._data,
                    },
                )?
                .to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }

    fn __pow__(lhs: PyRef<'p, Self>, rhs: i32, _mod: Option<u32>) -> Self {
        Self {
            _data: lhs._data.powi(rhs),
        }
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok(Self { _data: -self._data })
    }
}

#[pymethods]
impl PySINumber {
    pub fn sqrt(&self) -> Result<Self, QuantityError> {
        Ok(Self {
            _data: self._data.sqrt()?,
        })
    }

    pub fn cbrt(&self) -> Result<Self, QuantityError> {
        Ok(Self {
            _data: self._data.cbrt()?,
        })
    }
}
