## How it works: values and units

A quantity consists of two parts: a *value* and a *unit*.
Internally, a unit is represented by a list of seven integers, one for each SI base unit:

```py
unit: list[int] # [meter, kilogram, second, ampere, mol, kelvin, candela]
```

Each value in this list corresponds to the *exponent* of the respective unit.
While it is possible to construct a `SIObject` by providing a value and a unit, 
it is not recommendend because it is not very readable and prone to error:
```py linenums="1"
import si_units as si
mass = si.SIObject(125.0, [0, 1, 0, 0, 0, 0, 0])
print(mass)
```
```
125 kg
```

Instead, we can use one of the "units" (a `SIObject` with value one) 
defined in the package and multiply it by the value. 
Here we use `KILOGRAM` to yield the same result as above:

```py linenums="1"
import si_units as si
mass = 125.0 * si.KILOGRAM
print(mass)
```
```
125 kg
```

For convenience and readability, `si_units` also defines *prefixes*.
The following again yields the same result using the `KILO`
prefix together with `GRAM`:

```py linenums="1"
import si_units as si
mass = 125.0 * si.KILO * si.GRAM
print(mass)
```
```
125 kg
```

Prefixes and units can be used to define new types that you may want 
to reuse across your code:
```py linenums="1"
import si_units as si
MPA = si.MEGA * si.PASCAL
KG_M3 = si.KILOGRAM / si.METER**3
FS = si.FEMTO * si.SECOND
...
```


## Unit conversion

Consider the pressure of an ideal gas:

```py linenums="1"
from si_units import *
temperature = 298.15 * KELVIN
volume = 1.5 * METER**3
moles = 75.0 * MOL
pressure = moles * RGAS * temperature / volume
print(pressure)
```

```
123.94785148011941 kPa
```

Internally, all `SIObject`s are represented in base SI units.
Dividing by a unit (including prefixes) yields the value which can be a 
scalar or an array or tensor. This can be used to "convert" a quantity
into a value of the desired unit:

```py linenums="1" hl_lines="6-8"
import si_units as si
temperature = 298.15 * si.KELVIN
volume = 1.5 * si.METER**3
moles = 75.0 * si.MOL
pressure = moles * si.RGAS * temperature / volume
print('pressure / bar:   ', pressure / si.BAR)
print('pressure / mN/A^2:', pressure / (si.MILLI * si.NEWTON / si.ANGSTROM**2))
print('volume / l:       ', volume / si.LITER)
```

```
pressure / bar:    1.2394785148011942
pressure / mN/A^2: 1.239478514801194e-12
volume / l:        1500.0
```

## One-dimensional array constructors

There are several ways to construct a `SIObject` where the value
is a one-dimensional array.

```py linenums="1"
import si_units as si
# from a list of SIObjects
temperatures = si.array([298.15 * si.KELVIN, 313.15 * si.KELVIN])
# linearly spaced
heights = si.linspace(15 * si.CENTI * si.METER, 15 * si.METER, 1000)
# log-spaced
pressures = si.logspace(100 * si.KILO * si.PASCAL, 50 * si.BAR, 100)
```

For multi-dimensional arrays, first construct a `numpy.ndarray` of the
desired shape and multiply it by an unit.

## Gravitational pull of the moon on the earth

```py linenums="1"
import si_units as si
mass_earth = 5.9724e24 * si.KILOGRAM
mass_moon = 7.346e22 * si.KILOGRAM
distance = 383.398 * si.KILO * si.METER
force = si.G * mass_earth * mass_moon / distance**2
print(force)
```

```
1.992075748302325e26  N
```

## Pressure distribution in the atmosphere

Using the barometric formula. 
This example demonstrates how dimensioned arrays can be constructed 
using `numpy.ndarray`s. 

```py linenums="1"
import si_units as si
import numpy as np

z = np.linspace(1.0, 70.0e3, 10) * si.METER
g = 9.81 * si.METER / si.SECOND**2
m = 28.949 * si.GRAM / si.MOL
t = 283.15 * si.KELVIN
p0 = si.BAR
pressure = p0 * np.exp((-z * m * g) / (si.RGAS * t))

# dividing with the unit of an SIObject returns a numpy.ndarray
# iteration is currently not implemented.
for zi, pi in zip(z / si.METER, pressure / (si.KILO * si.PASCAL)):
    print(f'z = {zi:16.10f}   p = {pi:16.10f}')
```

```title="Output"
z =     1.0000000000   p =    99.9879378249
z =  7778.6666666667   p =    39.1279560236
z = 15556.3333333333   p =    15.3118163640
z = 23334.0000000000   p =     5.9919235296
z = 31111.6666666667   p =     2.3448000375
z = 38889.3333333333   p =     0.9175830080
z = 46667.0000000000   p =     0.3590747881
z = 54444.6666666667   p =     0.1405155744
z = 62222.3333333333   p =     0.0549875048
z = 70000.0000000000   p =     0.0215180823
```

Alternatively, we could have used `si_units.linspace` instead of numpy:
```py linenums="1"
import si_units as si
# instead of this
z = np.linspace(1.0, 70.0e3, 10) * si.METER
# we can also use this
z = si.linspace(1.0 * si.METER, 70.0e3 * si.METER, 10)
```

## Using `numpy` or `torch` functions

Some functions work with methods or the equivalent numpy functions.

```py linenums="1"
import si_units as si
import numpy as np

sqm = si.METER**2
print(np.sqrt(sqm))
print(sqm.sqrt())   # this is equivalent
```

```
1  m
1  m
```

Some behaviour is not as you would expect. For example, when we
change the above to an array, numpy will throw an exception:
```py linenums="1"
import si_units as si
import numpy as np
sqm = np.array([1.0, 2.0]) * si.METER**2
print(np.sqrt(sqm))
print(sqm.sqrt())  # both calls raise an exception
```
```
AttributeError: 'numpy.ndarray' object has no attribute 'sqrt'
```

In such a case, we can divide by the unit to return the inner data type,
perform the operation to the value and the unit separately, and finally 
multiply by the unit to get back a `SIObject`.

For `torch.tensor`'s this is not an issue and the following works just 
fine:

```py linenums="1"
import si_units as si
import torch
ms = torch.tensor([2.0, 3.0, 4.0]) * si.METER
sqms = ms**2
print(sqms)
print(sqms.sqrt())
```

```
tensor([ 4.,  9., 16.]) m²
tensor([2., 3., 4.]) m
```
