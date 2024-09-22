use super::{Quantity, SIUnit};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{marker::PhantomData, sync::LazyLock};
use typenum::Integer;

static PYSIQUANTITY: LazyLock<PyObject> = LazyLock::new(|| {
    Python::with_gil(|py| {
        PyModule::import_bound(py, "si_units")
            .unwrap()
            .getattr("PySIQuantity")
            .unwrap()
            .unbind()
    })
});

impl<T: Integer, L: Integer, M: Integer, I: Integer, THETA: Integer, N: Integer, J: Integer>
    IntoPy<PyObject> for Quantity<f64, SIUnit<T, L, M, I, THETA, N, J>>
{
    fn into_py(self, py: Python<'_>) -> PyObject {
        let unit = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        PYSIQUANTITY
            .call_method1(py, "_from_raw_parts", (self.0, unit))
            .unwrap()
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
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (value, unit_from) = ob
            .call_method0("_into_raw_parts")?
            .extract::<(f64, [i8; 7])>()?;
        let unit_into = [L::I8, M::I8, T::I8, I::I8, N::I8, THETA::I8, J::I8];
        if unit_into == unit_from {
            Ok(Quantity(value, PhantomData))
        } else {
            Err(PyErr::new::<PyValueError, _>(
                "`identifier` must be a SMILES code or `Identifier` object.".to_string(),
            ))
        }
    }
}
