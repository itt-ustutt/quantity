# Welcome to `si-units`

This package provides representations of quantities with SI units.
It is written with flexibility in mind and is able to represent arbitrarily complex units.
In addition to simple scalar quantities, it can be used to decorate any complex data type (numpy arrays, PyTorch tensors) to provide unit checks.

## Installation

You can install `si-units` from pypi using `pip`:

```
pip install si-units
```

## Build from source

To build the code from source, you need the [rust compiler](https://www.rust-lang.org/tools/install).

### maturin

With [maturin](https://github.com/PyO3/maturin) installed:

```
git clone https://github.com/itt-ustutt/quantity
cd quantity/si-units
maturin build --release
```

You can then install the generated wheel (placed in `quantity/target/wheels`) into your Python environment.

### uv

You can use [uv](https://github.com/astral-sh/uv) to build the wheel:

```
git clone https://github.com/itt-ustutt/quantity
cd quantity/si-units
uv build
```

You can then install the generated wheel (placed in `dist/`) into your Python environment.

## Usage

```py title="Ideal gas pressure" linenums="1"
from si_units import *
temperature = 25.0 * CELSIUS
volume = 1.5 * METER**3
moles = 75.0 * MOL
pressure = moles * RGAS * temperature / volume
print(pressure)
```

```
123.94785148011941 kPa
```

This also works with `numpy.ndarray`:

```py title="Using numpy" linenums="1"
from si_units import *
import numpy as np
ms = np.linspace(2.0, 4.0, 3) * METER
sqms = ms**2
print(sqms)
```

```
[4, 9, 16] m²
```

When you divide a quantity by its unit, the value
(i.e. `float` for scalars or `numpy.ndarray[float]` for arrays) is returned.
You can use this to convert units:

```py title="Unit conversion" linenums="1"
from si_units import MOL, METER, ANGSTROM, NAV

molar_density = 3000 * MOL / METER**3
particle_density = molar_density * NAV
nparticles = 16_000
volume = nparticles / particle_density

# make sure we actually have a volume
# this checks the quantity (here length**3), not the actual unit
# i.e. we could have used volume.has_unit(METER**3) for the same effect.
assert volume.has_unit(METER**3), "Something went wrong."
print(f'V = {volume / ANGSTROM**3:.2g} A^3')
```

```
V = 8.9e+06 A^3
```

See [Examples](examples.md) section for more use cases.