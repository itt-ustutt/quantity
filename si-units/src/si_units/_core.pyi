from typing import Self, Any

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
            by the appropriate unit.

        Args:
            value:
                The numerical value(s). Can be a scalar or an array
                such as a numpy.ndarray or a torch.tensor.
            unit: List of 7 exponents for SI base units.

        Raises:
            RuntimeError: When unit has the wrong format.
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
            >>> volume = METER**2
            >>> length = volume.sqrt()
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

class SIArray1:
    """Builder for SIObjects with numpy.narray as value storage."""

    def __call__(self, value: SIObject | list[SIObject]) -> SIObject:
        """Build SIObject from scalar or list.

        Args:
            value: Values to store. Must all have the same unit.

        Returns:
            The quantity with values stored within array, even
            if value is given as a scalar.

        Raises:
            RuntimeError: If the elements of value have different units.
        """
        ...

    @staticmethod
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
                If start and end values are not scalars and if they don't have
                the same unit.
                If n is not positive.
        """
        ...

    @staticmethod
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
                If start and end values are not scalars and if they don't have
                the same unit.
                If n is not positive.
        """
        ...
