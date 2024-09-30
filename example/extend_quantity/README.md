# Example: Extend the quantity and si-units packages
This example crate shows how to build a rust/PyO3 crate that uses the data types from the [quantity](../README.md) crate in Rust to build a Python package. In the Python interface, data types are converted to classes from the [si-units](../si-units/README.md) Python package.

## Installation
```
maturin develop --release
```

## Examples
```python
>>> from extend_quantity import ideal_gas
>>> from si_units import KELVIN, LITER, MOL
>>> ideal_gas(300 * KELVIN, 5 * LITER, 10 * MOL)
4.9886775708919435 MPa
```
Or using numpy arrays

```python
>>> from extend_quantity import ideal_gas_array
>>> from si_units import CELSIUS, METER, MOL, SIArray1
>>> import numpy as np
>>> T = np.array([25, 45]) * CELSIUS
>>> V = np.array([3, 5]) * METER**3
>>> N = SIArray1.linspace(MOL, 2*MOL, 2)
>>> ideal_gas_array(T, V, N)
[826.3190098674627, 1058.0985127861811] Pa
```
