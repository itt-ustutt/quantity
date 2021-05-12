#[macro_use]
macro_rules! impl_array {
    ($struct:ident, $core_struct:ty, $numpy_array:ty) => {
        impl From<$core_struct> for $struct {
            fn from(si: $core_struct) -> Self {
                Self { _data: si }
            }
        }

        impl From<$struct> for $core_struct {
            fn from(si: $struct) -> Self {
                si._data
            }
        }

        #[pymethods]
        impl $struct {
            #[classattr]
            fn __array_priority__() -> u64 {
                100
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
            fn has_unit(&self, other: Self) -> bool {
                self._data.has_unit(&other._data)
            }

            fn as_list(&self) -> Vec<PySINumber> {
                self._data
                    .to_vec()
                    .iter()
                    .map(|v| PySINumber { _data: *v })
                    .collect()
            }

            #[getter]
            fn get_shape(&self) -> Vec<usize> {
                self._data.shape().to_vec()
            }
        }

        #[pyproto]
        impl pyo3::class::basic::PyObjectProtocol for $struct {
            fn __repr__(&self) -> PyResult<String> {
                Ok(self._data.to_string())
            }
        }

        #[pyproto]
        impl PySequenceProtocol for $struct {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self._data.len())
            }
        }

        #[pyproto]
        impl PyNumberProtocol for $struct {
            fn __add__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(r) = rhs.extract::<Self>() {
                        if lhs._data.has_unit(&r._data) {
                            return Ok(PyCell::new(
                                py,
                                Self {
                                    _data: lhs._data.clone() + r._data,
                                },
                            )?
                            .to_object(py));
                        } else {
                            return Err(PyErr::new::<PyTypeError, _>(format!(
                                "incompatible units."
                            )));
                        };
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }

            fn __sub__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(r) = rhs.extract::<Self>() {
                        if lhs._data.has_unit(&r._data) {
                            return Ok(PyCell::new(
                                py,
                                Self {
                                    _data: lhs._data.clone() - r._data,
                                },
                            )?
                            .to_object(py));
                        } else {
                            return Err(PyErr::new::<PyTypeError, _>(format!(
                                "incompatible units."
                            )));
                        };
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }

            fn __mul__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(r) = rhs.extract::<Self>() {
                        let result = lhs._data.clone() * r._data;
                        return Ok(match result.value() {
                            Ok(r) => r.to_pyarray(py).to_object(py),
                            Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                        });
                    };
                    if let Ok(r) = rhs.extract::<$numpy_array>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: lhs._data.clone() * r.to_owned_array(),
                            },
                        )?
                        .to_object(py));
                    };
                    // f64
                    if let Ok(r) = rhs.extract::<f64>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: lhs._data.clone() * r,
                            },
                        )?
                        .to_object(py));
                    };
                    // SINumber
                    if let Ok(r) = rhs.extract::<PySINumber>() {
                        let result = lhs._data.clone() * r._data;
                        return Ok(match result.value() {
                            Ok(r) => r.to_pyarray(py).to_object(py),
                            Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                        });
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }

            fn __rmul__(&self, lhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(l) = lhs.extract::<$numpy_array>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: l.to_owned_array() * self._data.clone(),
                            },
                        )?
                        .to_object(py));
                    };
                    if let Ok(l) = lhs.extract::<f64>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: self._data.clone() * l,
                            },
                        )?
                        .to_object(py));
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }

            fn __truediv__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(r) = rhs.extract::<Self>() {
                        let result = lhs._data.clone() / r._data;
                        return Ok(match result.value() {
                            Ok(r) => r.to_pyarray(py).to_object(py),
                            Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                        });
                    };
                    if let Ok(r) = rhs.extract::<$numpy_array>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: lhs._data.clone() * (1.0 / r.to_owned_array()),
                            },
                        )?
                        .to_object(py));
                    };
                    // f64
                    if let Ok(r) = rhs.extract::<f64>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: lhs._data.clone() / r,
                            },
                        )?
                        .to_object(py));
                    };
                    // SINumber
                    if let Ok(r) = rhs.extract::<PySINumber>() {
                        let result = lhs._data.clone() / r._data;
                        return Ok(match result.value() {
                            Ok(r) => r.to_pyarray(py).to_object(py),
                            Err(_) => PyCell::new(py, Self { _data: result })?.to_object(py),
                        });
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }

            fn __rtruediv__(&self, lhs: &PyAny) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    if let Ok(l) = lhs.extract::<$numpy_array>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: 1.0 / self._data.clone() * l.to_owned_array(),
                            },
                        )?
                        .to_object(py));
                    };
                    if let Ok(l) = lhs.extract::<f64>() {
                        return Ok(PyCell::new(
                            py,
                            Self {
                                _data: l / self._data.clone(),
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
                Ok(Self {
                    _data: -self._data.clone(),
                })
            }
        }

        #[pymethods]
        impl $struct {
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
    };
}
