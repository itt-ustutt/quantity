Welcome to si's documentation!
==============================

Examples
--------

Pressure of an ideal gas
~~~~~~~~~~~~~~~~~~~~~~~~

    >>> from quantity import *
    >>> temperature = 298.15 * KELVIN
    >>> volume = 1.5 * METER**3
    >>> moles = 75.0 * MOL
    >>> pressure = moles * RGAS * temperature / volume
    >>> print(pressure)
    123.94785148011941 kPa

You can use division to perform unit conversions.

    >>> from quantity import *
    >>> temperature = 298.15 * KELVIN
    >>> volume = 1.5 * METER**3
    >>> moles = 75.0 * MOL
    >>> pressure = moles * RGAS * temperature / volume
    >>> print(pressure / BAR)
    1.2394785148011942
    >>> print(pressure / (MILLI * NEWTON / ANGSTROM**2))
    1.239478514801194e-12
    >>> print(volume / LITER)
    1500.0

Gravitational pull of the moon on the earth
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    >>> from quantity import *
    >>> mass_earth = 5.9724e24 * KILOGRAM
    >>> mass_moon = 7.346e22 * KILOGRAM
    >>> distance = 383.398 * KILO * METER
    >>> force = G * mass_earth * mass_moon / distance**2
    >>> print(force)
    199.20757483023252 YN

Pressure distribution in the atmosphere
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Using the barometric formula.
This example demonstrates how dimensioned arrays can be constructed using numpy.ndarray's.

.. code-block:: python

    >>> from quantity import *
    >>> import numpy as np

    >>> z = np.linspace(1.0, 70.0e3, 10) * METER
    >>> g = 9.81 * METER / SECOND**2
    >>> m = 28.949 * GRAM / MOL
    >>> t = 283.15 * KELVIN
    >>> p0 = BAR
    >>> pressure = p0 * np.exp((-z * m * g) / (RGAS * t))
    >>>
    >>> # dividing with the unit of an SIArray returns a numpy.ndarray
    >>> # iteration is currently not implemented.
    >>> for zi, pi in zip(z / METER, pressure / (KILO * PASCAL)):
    >>>     print(f'z = {zi:16.10f}   p = {pi:16.10f}')

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

Using `numpy` Functions
~~~~~~~~~~~~~~~~~~~~~~~

Functions such as `exp`, `sqrt` and `cbrt` work with methods or the equivalent numpy functions.

    >>> from quantity import *
    >>> import numpy as np
    >>> sqm = METER**2
    >>> np.sqrt(sqm)
    1  m
    >>> sqm.sqrt()   # this is equivalent
    1  m

This also works with numpy.ndarray's.

    >>> from quantity import *
    >>> import numpy as np
    >>> ms = np.array([2.0, 3.0, 4.0]) * METER
    >>> sqms = np.square(ms)
    >>> sqms
    [4, 9, 16] m^2
    >>> np.sqrt(sqms)
    [2, 3, 4] m

* :ref:`search`
