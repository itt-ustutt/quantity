use ndarray::Array1;
use pyo3::prelude::*;
use quantity::*;

#[pyfunction]
fn bar() -> Pressure {
    BAR
}

#[pyfunction]
fn ideal_gas(temperature: Temperature, volume: Volume, moles: Moles) -> Pressure {
    moles * RGAS * temperature / volume
}

#[pyfunction]
fn ideal_gas_array(
    temperature: Temperature<Array1<f64>>,
    volume: Volume<Array1<f64>>,
    moles: Moles<Array1<f64>>,
) -> Pressure<Array1<f64>> {
    moles * RGAS * temperature / volume
}

#[pymodule]
fn extend_quantity(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bar, m)?)?;
    m.add_function(wrap_pyfunction!(ideal_gas, m)?)?;
    m.add_function(wrap_pyfunction!(ideal_gas_array, m)?)?;
    Ok(())
}
