macro_rules! impl_array {
    ($struct:ident, $core_struct:ty, $numpy_array:ty) => {
        impl From<$core_struct> for $struct {
            fn from(si: $core_struct) -> Self {
                Self(si)
            }
        }

        impl From<$struct> for $core_struct {
            fn from(si: $struct) -> Self {
                si.0
            }
        }

        impl Deref for $struct {
            type Target = $core_struct;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[pymethods]
        impl $struct {
            fn __setstate__(&mut self, state: &Bound<'_, PyBytes>) {
                self.0 = deserialize(state.as_bytes()).unwrap();
            }

            fn __getstate__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
                PyBytes::new_bound(py, &serialize(&self.0).unwrap())
            }

            pub fn sqrt(&self) -> Result<Self, QuantityError> {
                Ok(Self(self.0.sqrt()?))
            }

            pub fn cbrt(&self) -> Result<Self, QuantityError> {
                Ok(Self(self.0.cbrt()?))
            }

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
            fn has_unit(&self, other: Self) -> bool {
                self.0.has_unit(&other.0)
            }

            fn as_list(&self) -> Vec<PySINumber> {
                self.0.to_vec().iter().map(|v| PySINumber(*v)).collect()
            }

            #[getter]
            fn get_shape(&self) -> Vec<usize> {
                self.0.shape().to_vec()
            }

            fn __repr__(&self) -> PyResult<String> {
                Ok(self.0.to_string())
            }

            fn __add__<'py>(&self, rhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(r) = rhs.extract::<PySINumber>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() + r.0))?.into_any());
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() + r.0))?.into_any());
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __sub__<'py>(&self, rhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(r) = rhs.extract::<PySINumber>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() - r.0))?.into_any());
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() - r.0))?.into_any());
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __mul__<'py>(&self, rhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() * r))?.into_any());
                };
                if let Ok(r) = rhs.extract::<PySINumber>() {
                    let result = self.0.clone() * r.0;
                    return Ok(match result.value() {
                        Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                        Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
                    });
                };
                if let Ok(r) = rhs.extract::<$numpy_array>() {
                    return Ok(
                        Bound::new(rhs.py(), Self(self.0.clone() * r.to_owned_array()))?.into_any(),
                    );
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    let result = self.0.clone() * r.0;
                    return Ok(match result.value() {
                        Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                        Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __rmul__<'py>(&self, lhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(l) = lhs.extract::<f64>() {
                    return Ok(Bound::new(lhs.py(), Self(self.0.clone() * l))?.into_any());
                };
                if let Ok(l) = lhs.extract::<$numpy_array>() {
                    return Ok(
                        Bound::new(lhs.py(), Self(l.to_owned_array() * self.0.clone()))?.into_any(),
                    );
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __truediv__<'py>(&self, rhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Bound::new(rhs.py(), Self(self.0.clone() / r))?.into_any());
                };
                if let Ok(r) = rhs.extract::<PySINumber>() {
                    let result = self.0.clone() / r.0;
                    return Ok(match result.value() {
                        Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                        Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
                    });
                };
                if let Ok(_) = rhs.extract::<PyCelsius>() {
                    return Ok((self.0.clone() / CELSIUS)
                        .to_pyarray_bound(rhs.py())
                        .into_any());
                };
                if let Ok(r) = rhs.extract::<$numpy_array>() {
                    return Ok(Bound::new(
                        rhs.py(),
                        Self(self.0.clone() * (1.0 / r.to_owned_array())),
                    )?
                    .into_any());
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    let result = self.0.clone() / r.0;
                    return Ok(match result.value() {
                        Ok(r) => r.to_pyarray_bound(rhs.py()).into_any(),
                        Err(_) => Bound::new(rhs.py(), Self(result))?.into_any(),
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __rtruediv__<'py>(&self, lhs: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
                if let Ok(l) = lhs.extract::<f64>() {
                    return Ok(Bound::new(lhs.py(), Self(l / self.0.clone()))?.into_any());
                };
                if let Ok(l) = lhs.extract::<$numpy_array>() {
                    return Ok(Bound::new(
                        lhs.py(),
                        Self(1.0 / self.0.clone() * l.to_owned_array()),
                    )?
                    .into_any());
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __pow__(&self, rhs: i32, _mod: Option<u32>) -> Self {
                Self(self.0.powi(rhs))
            }

            fn __neg__(&self) -> Self {
                Self(-&self.0)
            }
        }
    };
}
