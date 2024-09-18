use crate::si::SINumber;
use bincode::{deserialize, serialize};
use std::sync::LazyLock;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub(crate) static QUANTITY: LazyLock<PyObject> = LazyLock::new(|| {
    Python::with_gil(|py| {
        PyModule::import_bound(py, "si_units")
            .unwrap()
            .to_object(py)
    })
});

pub(crate) static SINUMBER: LazyLock<PyObject> =
    LazyLock::new(|| Python::with_gil(|py| QUANTITY.getattr(py, "SINumber").unwrap()));

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PySINumber(pub SINumber);

impl<'py> FromPyObject<'py> for PySINumber {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let state: Bound<'py, PyBytes> = ob
            .call_method0("__getstate__")?
            .extract::<Bound<'py, PyBytes>>()?;
        Ok(Self(deserialize(state.as_bytes()).unwrap()))
    }
}

impl IntoPy<PyObject> for PySINumber {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let state = serialize(&self.0).unwrap();
        SINUMBER
            .bind(py)
            .call_method1("_from_state", (PyBytes::new_bound(py, &state),))
            .unwrap()
            .into_py(py)
    }
}
