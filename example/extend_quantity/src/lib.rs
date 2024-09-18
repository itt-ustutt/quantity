use pyo3::prelude::*;
use quantity::pyo3::PySINumber;

#[pyfunction]
fn power(number: PySINumber, i: i32) -> PySINumber {
    PySINumber(number.0.powi(i))
}

#[pymodule]
fn extend_quantity(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(power, m)?)?;
    Ok(())
}
