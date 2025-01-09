# Example: Extend the quantity and si-units packages
This example crate shows how to build a rust/PyO3 crate that uses the data types from the [quantity](../../README.md) crate in Rust to build a Python package. In the Python interface, data types are converted to classes from the [si-units](../../si-units/README.md) Python package.

## Installation
```
maturin develop --release
```

## Examples
```python
from extend_quantity import ideal_gas
import si_units as si
print(ideal_gas(300 * si.KELVIN, 5 * si.LITER, 10 * si.MOL))
```
```
4.9886775708919435 MPa
```

Or using numpy arrays

```python
from extend_quantity import ideal_gas_array
import si_units as si
import numpy as np
T = np.array([25, 45]) * si.CELSIUS
V = np.array([3, 5]) * si.METER**3
N = si.linspace(si.MOL, 2*si.MOL, 2)
print(ideal_gas_array(T, V, N))
```
```
array([ 826.31900987, 1058.09851279]) Pa
```