use pyo3::prelude::*;
use quantity::python::quantity;

/// Implementation of SI numbers.
#[pymodule]
pub fn si_units(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    quantity(_py, m)
}
