def test_scalar():
    """
    >>> from extend_quantity import ideal_gas
    >>> import si_units as si
    >>> ideal_gas(300 * si.KELVIN, 5 * si.LITER, 10 * si.MOL)
    4.9886775708919435 MPa
    """


def test_array():
    """
    >>> from extend_quantity import ideal_gas_array
    >>> import si_units as si
    >>> import numpy as np
    >>> T = np.array([25, 45]) * si.CELSIUS
    >>> V = np.array([3, 5]) * si.METER**3
    >>> N = si.linspace(si.MOL, 2*si.MOL, 2)
    >>> ideal_gas_array(T, V, N)
    array([ 826.31900987, 1058.09851279]) Pa
    """
