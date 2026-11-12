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

def test_nalgebra():
    """
    >>> from extend_quantity import test_nalgebra
    >>> import si_units as si
    >>> import numpy as np
    >>> p = np.array([[1, 2], [3, 4]]) * si.BAR
    >>> V = np.array([1, 2]) * si.LITER
    >>> test_nalgebra(p, V)
    array([ 500., 1100.]) J
    """

def test_repr_array():
    """
    >>> import si_units as si
    >>> import numpy as np
    >>> repr(np.array([1.0, 2.0, 3.0]) * si.GRAM)
    'array([1., 2., 3.]) g'
    >>> repr(np.array([1.0, 2.0, 3.0]) * si.GRAM / si.MOL)
    'array([1., 2., 3.]) g/mol'
    >>> repr(np.array([1.0, 2.0, 3.0]) * si.GRAM / si.METER**3)
    'array([1., 2., 3.]) g/m³'
    >>> repr(np.array([1.0, 2.0, 3.0]) * si.METER)
    'array([1., 2., 3.]) m'
    """
