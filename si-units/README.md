# si-units

[![documentation](https://img.shields.io/badge/docs-github--pages-blue)](https://itt-ustutt.github.io/quantity/index.html)
[![PyPI version](https://badge.fury.io/py/si_units.svg)](https://badge.fury.io/py/si_units)

Representation of quantities with SI units. 

The package is written with flexibility in mind and is able to represent arbitrarily complex units.
In addition to simple scalar quantities, it can be used to decorate any complex data type (numpy arrays, PyTorch tensors) to provide unit checks.

## Installation and Usage

You can install the python package from source (you need a rust compiler for that):

```
pip install git+https://github.com/itt-ustutt/quantity
```

or get the compiled files from PyPI

```
pip install si-units
```

## Examples

Calculate the pressure of an ideal gas.

```python
from si_units import *
temperature = 25.0 * CELSIUS
volume = 1.5 * METER**3
moles = 75.0 * MOL
pressure = moles * RGAS * temperature / volume
print(pressure) # 123.94785148011941 kPa
```

`numpy` functions can be used with SI units:

```python
from si_units import *
import numpy as np
ms = np.linspace(2.0, 4.0, 3) * METER
sqms = ms**2
print(sqms) # [4, 9, 16] m²
```

## Documentation

For the documentation, see [here](https://itt-ustutt.github.io/quantity/index.html).
