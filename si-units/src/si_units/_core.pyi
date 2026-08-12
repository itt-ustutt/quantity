from typing import Any, Final, Self

class SIObject:
    """Combination of value and unit.

    The value can be any Python object that can be used for arithmetic
    operations such as a float, numpy.ndarray or torch.tensor.

    When a SIObject is divided by its unit, the value is returned.
    This is usefull to convert units or when operations are needed
    that are not implemented for SIObject.
    """

    def __init__(self, value: float | Any, unit: list[int]) -> None:
        """Constructs a new quantity.

        Warning: Don't use the default constructor
            This constructor should not be used to construct a quantity.
            Instead, multiply the value (float or array of floats)
            by the appropriate unit. See example below.

        Args:
            value:
                The numerical value(s). Can be a scalar or an array
                such as a numpy.ndarray or a torch.tensor.
            unit: List of 7 exponents for SI base units.

        Raises:
            RuntimeError: When unit has the wrong format.

        Examples:
            >>> import si_units as si
            >>> # don't do this:
            >>> two_meters_init = si.SIObject(2.0, [1, 0, 0, 0, 0, 0, 0])
            >>> # instead, do this:
            >>> two_meters_mul = 2.0 * si.METER
            >>> assert two_meters_init == two_meters_mul
        """
        ...

    def sqrt(self) -> Self:
        """Calculates the square root.

        Returns:
            Square root of the quantity.

        Raises:
            RuntimeError: When exponents of units are not multiples of two.
            AttributeError: When the inner data type has no 'sqrt' method.

        Examples:
            >>> from si_units import METER
            >>> square = METER**2
            >>> length = square.sqrt()
        """
        ...

    def cbrt(self) -> Self:
        """Calculate the cubic root.

        Returns:
            Cubic root of the quantity.

        Raises:
            RuntimeError: When exponents of units are not multiples of three.
            AttributeError: When the inner data type has no 'cbrt' method.

        Examples:
            >>> from si_units import METER
            >>> volume = METER**3
            >>> length = volume.cbrt()
        """
        ...

    def has_unit(self, other: Self) -> bool:
        """Tests if the quantity has the same unit as the argument.

        Args:
            other: The quantity to compare to.

        Returns:
            Wheter the units of the compared quantities are the same or not.
        """
        ...

    def value_in(self, unit: Self) -> float | Any:
        """Return the numeric value expressed in specified unit.

        The underlying value (float, numpy.ndarray, torch.tensor, ...)
        is divided by unit and returned without the unit wrapper.

        Args:
            unit: A quantity describing target unit (e.g. KILO * WATT * HOUR).

        Returns:
            The numeric value of self expressed in unit.

        Raises:
            RuntimeError: When self and unit have incompatible units.

        Examples:
            >>> from si_units import JOULE, KILO, WATT, HOUR
            >>> energy = 5.4e6 * JOULE
            >>> energy.value_in(KILO * WATT * HOUR)
            1.5
        """
        ...

    __array_priority__: int
    value: Any
    @property
    def unit(self) -> list[int]: ...
    @property
    def shape(self) -> Any: ...
    def __getnewargs__(self) -> tuple[Any, list[int]]: ...
    def __repr__(self) -> str: ...
    def _repr_latex_(self) -> str | None: ...
    def __eq__(self, other: object) -> Any: ...
    def __ne__(self, other: object) -> Any: ...
    def __lt__(self, other: Self) -> Any: ...
    def __le__(self, other: Self) -> Any: ...
    def __gt__(self, other: Self) -> Any: ...
    def __ge__(self, other: Self) -> Any: ...
    def __add__(self, rhs: Self) -> Self: ...
    def __sub__(self, rhs: Self) -> Self: ...
    def __mul__(self, rhs: Any) -> Any: ...
    def __rmul__(self, lhs: Any) -> Any: ...
    def __truediv__(self, rhs: Any) -> Any: ...
    def __rtruediv__(self, lhs: Any) -> Any: ...
    def __pow__(self, i: int, _mod: object | None = None) -> Self: ...
    def __neg__(self) -> Self: ...
    def __abs__(self) -> Self: ...
    def __len__(self) -> int: ...
    def __getitem__(self, idx: int) -> Self: ...
    def __setitem__(self, idx: int, value: Self) -> None: ...

def array(value: SIObject | list[SIObject]) -> SIObject:
    """Build SIObject from scalar or list.

    When the input is a scalar, it is stored in an array with a single element.

    Args:
        value: Values to store. Must all have the same unit.

    Returns:
        The quantity with values stored within array,
            even if value is given as a scalar.

    Raises:
        RuntimeError: If the elements of value have different units.
    """
    ...

def linspace(start: SIObject, end: SIObject, n: int) -> SIObject:
    """Linearly spaced quantities.

    Args:
        start: Lowest value.
        end: Highest value.
        n: The (positive) number of points.

    Returns:
        Linearly spaced values with the same unit.

    Raises:
        RuntimeError:
            If start and end values are not scalars, if they don't have
            the same unit, or if n is not positive.
    """
    ...

def declare_unit(value: float, expr: str) -> SIObject | float:
    """Build a quantity from a numeric value and an SI unit expression.

    The expression accepts SI base and derived unit symbols, optionally
    combined with standard SI prefixes (e.g. `"kWh"`, `"mm**2"`, `"J/mol/K"`).
    Each atom is matched greedily; ties between a bare unit and a prefix + unit
    split go to the bare unit, so `kg`, `cd`, `mol`, `min`, `Pa` keep their
    canonical meaning.

    Recognised symbols (case-sensitive):

    * SI base/derived: `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `Hz`, `N`,
      `Pa`, `J`, `W`, `C`, `V`, `F`, `S`, `T`, `H`, `Wb`.
    * Accepted non-SI: `min`, `h`, `d`, `bar`, `cal`, `g`, `L`/`l`.
    * Special units with ASCII aliases: `Ohm`/`ohm` (also `Ω`, U+03A9),
      `Ang`/`ang` (also `Å`, U+00C5).
    * Prefixes: `Q R Y Z E P T G M k h d c m u n p f a z y r q`, plus
      `da` for deca and `µ` (U+00B5) as alias for `u` (micro).

    Supported operators: `*`, `/`, `**n` (or `^n`), with `²` (U+00B2) and `³`
    (U+00B3) as shortcuts. Whitespace and implicit multiplication between
    adjacent atoms are allowed.

    Args:
        value: Numeric value expressed in the given unit.
        expr: Unit expression string.

    Returns:
        A SIObject in base SI units, or a float when the expression is
        dimensionless.

    Raises:
        ValueError: If the expression cannot be parsed.

    Examples:
        >>> from si_units import declare_unit, KILO, WATT, HOUR
        >>> declare_unit(5.0, "kWh") == 5.0 * KILO * WATT * HOUR
        True
    """
    ...

def logspace(start: SIObject, end: SIObject, n: int) -> SIObject:
    """Logarithmically spaced quantities.

    Args:
        start: Lowest value.
        end: Highest value.
        n: The (positive) number of points.

    Returns:
        Logarithmically spaced values with the same unit.

    Raises:
        RuntimeError:
            If start and end values are not scalars, if they don't have
            the same unit, or if n is not positive.
    """
    ...

class Celsius:
    __array_priority__: int
    def __rmul__(self, lhs: Any) -> SIObject: ...

class Debye:
    def __rmul__(self, lhs: float) -> Self: ...
    def __pow__(self, n: int, _mod: object | None = None) -> SIObject: ...

class Angle:
    def __init__(self, value: float) -> None: ...
    def __getnewargs__(self) -> float: ...
    def __add__(self, rhs: Self) -> Self: ...
    def __sub__(self, rhs: Self) -> Self: ...
    def __mul__(self, rhs: float) -> Self: ...
    def __rmul__(self, lhs: float) -> Self: ...
    def __truediv__(self, rhs: Any) -> Any: ...
    def __neg__(self) -> Self: ...
    def value_in(self, unit: Self) -> float: ...
    def sin(self) -> float: ...
    def cos(self) -> float: ...
    def tan(self) -> float: ...

SECOND: Final[SIObject]
METER: Final[SIObject]
KILOGRAM: Final[SIObject]
AMPERE: Final[SIObject]
KELVIN: Final[SIObject]
CELSIUS: Final[Celsius]
DEBYE: Final[Debye]
DEGREES: Final[Angle]
RADIANS: Final[Angle]
DAY: Final[SIObject]
MOL: Final[SIObject]
CANDELA: Final[SIObject]
DVCS: Final[SIObject]
CLIGHT: Final[SIObject]
PLANCK: Final[SIObject]
QE: Final[SIObject]
KB: Final[SIObject]
NAV: Final[SIObject]
KCD: Final[SIObject]
HERTZ: Final[SIObject]
NEWTON: Final[SIObject]
PASCAL: Final[SIObject]
JOULE: Final[SIObject]
WATT: Final[SIObject]
COULOMB: Final[SIObject]
VOLT: Final[SIObject]
FARAD: Final[SIObject]
OHM: Final[SIObject]
SIEMENS: Final[SIObject]
WEBER: Final[SIObject]
TESLA: Final[SIObject]
HENRY: Final[SIObject]
ANGSTROM: Final[SIObject]
AMU: Final[SIObject]
AU: Final[SIObject]
BAR: Final[SIObject]
ATM: Final[SIObject]
POISE: Final[SIObject]
CALORIE: Final[SIObject]
GRAM: Final[SIObject]
HOUR: Final[SIObject]
LITER: Final[SIObject]
MINUTE: Final[SIObject]
G: Final[SIObject]
RGAS: Final[SIObject]
EPSILON0: Final[SIObject]
KE: Final[SIObject]
QUECTO: Final[float]
RONTO: Final[float]
YOCTO: Final[float]
ZEPTO: Final[float]
ATTO: Final[float]
FEMTO: Final[float]
PICO: Final[float]
NANO: Final[float]
MICRO: Final[float]
MILLI: Final[float]
CENTI: Final[float]
DECI: Final[float]
DECA: Final[float]
HECTO: Final[float]
KILO: Final[float]
MEGA: Final[float]
GIGA: Final[float]
TERA: Final[float]
PETA: Final[float]
EXA: Final[float]
ZETTA: Final[float]
YOTTA: Final[float]
RONNA: Final[float]
QUETTA: Final[float]
__version__: Final[str]
