use pyo3::prelude::*;
use quantity::pyo3::PySINumber;

#[pyfunction]
fn power(number: PySINumber, i: i32) -> PySINumber {
    PySINumber(number.0.powi(i))
}

#[pyfunction]
fn bar() -> Pressure<f64> {
    BAR
}

#[pyfunction]
fn ideal_gas(
    temperature: Temperature<f64>,
    volume: Volume<f64>,
    moles: Moles<f64>,
) -> Pressure<f64> {
    moles * RGAS * temperature / volume
}

#[pymodule]
fn extend_quantity(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(power, m)?)?;
    m.add_function(wrap_pyfunction!(bar, m)?)?;
    m.add_function(wrap_pyfunction!(ideal_gas, m)?)?;
    Ok(())
}

mod si;
use si::*;

use thiserror::Error;

/// Error type for improperly defined states and convergence problems.
#[derive(Error, Debug)]
pub enum EosError {
    #[error("{0}")]
    Error(String),
    #[error("`{0}` did not converge within the maximum number of iterations.")]
    NotConverged(String),
    #[error("`{0}` encountered illegal values during the iteration.")]
    IterationFailed(String),
    #[error("Iteration resulted in trivial solution.")]
    TrivialSolution,
    #[error("Equation of state is initialized for {0} components while the input specifies {1} components.")]
    IncompatibleComponents(usize, usize),
    #[error("Invalid state in {0}: {1} = {2}.")]
    InvalidState(String, String, f64),
    #[error("Undetermined state: {0}.")]
    UndeterminedState(String),
    #[error("System is supercritical.")]
    SuperCritical,
    #[error("No phase split according to stability analysis.")]
    NoPhaseSplit,
    #[error("Wrong input units. Expected {0}, got {1}")]
    WrongUnits(String, String),
}

/// Convenience type for `Result<T, EosError>`.
pub type EosResult<T> = Result<T, EosError>;
