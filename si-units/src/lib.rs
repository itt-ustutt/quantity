use pyo3::prelude::*;
use quantity::python::quantity;

/// Implementation of SI numbers.
#[pymodule]
pub fn si_units<'py>(py: Python<'py>, m: Bound<'py, PyModule>) -> PyResult<()> {
    quantity(py, m)
}
