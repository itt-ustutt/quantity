use pyo3::pymodule;

#[pymodule]
mod extend_quantity {
    use nalgebra::{DMatrix, DVector};
    use ndarray::Array1;
    use pyo3::pyfunction;
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

    #[pyfunction]
    fn law_of_cosines1(a: Length, b: Length, gamma: Angle) -> Length {
        (a * a + b * b - 2.0 * a * b * gamma.cos()).sqrt()
    }

    #[pyfunction]
    fn law_of_cosines2(a: Length, b: Length, c: Length) -> Angle {
        Angle::acos((a * a + b * b - c * c).convert_into(2.0 * a * b))
    }

    #[pyfunction]
    fn test_nalgebra(
        pressure: Pressure<DMatrix<f64>>,
        volume: Volume<DVector<f64>>,
    ) -> Energy<DVector<f64>> {
        pressure * volume
    }
}
