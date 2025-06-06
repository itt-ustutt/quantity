# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.5] - 2025-06-03
### Changed
- Made all methods (except of `logspace`) of quantity arrays generic with respect to the item type. [#88](https://github.com/itt-ustutt/quantity/pull/88)

## [0.10.4] - 2025-06-02
### Added
- Implemented `Display` for `Dimensionless<T>`. [#88](https://github.com/itt-ustutt/quantity/pull/88)
- Implemented `Debug` for `Angle<T>`. [#88](https://github.com/itt-ustutt/quantity/pull/88)
- Added  `Quantity<T, U>::inv`. [#88](https://github.com/itt-ustutt/quantity/pull/88)

### Changed
- Generalized implementation of `Quantity<T, U>::abs` to every type with `T: Signed`. [#88](https://github.com/itt-ustutt/quantity/pull/88)
- Generalized implementation of `Quantity::from_vec` and `Quantity::linspace`. [#88](https://github.com/itt-ustutt/quantity/pull/88)

## [0.10.3] - 2025-05-07
### Added
- Implemented `Deref` for `Dimensionless<T>`. [#86](https://github.com/itt-ustutt/quantity/pull/86)
- Added the new type aliases `MassFlux`, `HeatFlux`, `ThermalTransmittance`, and `ThermalResistance`. [#87](https://github.com/itt-ustutt/quantity/pull/87)

## [0.10.2] - 2025-04-09
### Added
- Added support for the new features in num-dual 0.11.1. [#83](https://github.com/itt-ustutt/quantity/pull/83)

## [0.10.1] - 2025-01-09
### Added
- Added formatting (and thus Python support) for heat capacity rates. [#81](https://github.com/itt-ustutt/quantity/pull/81)

## [0.10.0] - 2024-12-05
### Packaging
- Update `pyo3` and `numpy` dependencies to 0.23. [#76](https://github.com/itt-ustutt/quantity/pull/76)

## [0.9.1] - 2024-11-28
### Fixed
- Added division by `CELSIUS` in Python. [#74](https://github.com/itt-ustutt/quantity/pull/74)

## [0.9.0] - 2024-10-24
### Changed
- Reimplemented `si-units` Python package independent of `quantity` crate in and more "pythonic" fashion. [#63](https://github.com/itt-ustutt/quantity/pull/63)
- Reimplemented `quantity` to compile-time checked units using the `typenum` crate. [#64](https://github.com/itt-ustutt/quantity/pull/64)
- Hide non-essential dependencies behind features `ndarray`, `approx`, `num-dual`, `python`, `python_numpy`. [#70](https://github.com/itt-ustutt/quantity/pull/70)
- Implemented angles based on the `Quantity` struct rather than the `ang` crate. [#72](https://github.com/itt-ustutt/quantity/pull/72)

### Added
- Added basic support for the combination of structs in the `num-dual` crate with units. [#66](https://github.com/itt-ustutt/quantity/pull/66)

### Packaging
- Update `pyo3` and `numpy` dependencies to 0.22. [#65](https://github.com/itt-ustutt/quantity/pull/65)

## [0.8.0] - 2024-04-11
### Packaging
- Updated `pyo3` and `numpy` dependencies to 0.21 and adjusted to the new `Bound` API.

## [0.7.0] - 2023-10-15
### Packaging
- Updated `pyo3` and `numpy` dependencies to 0.20.
- Updated `approx` dependency to 0.5.

## [0.6.2] - 2023-08-06
### Added
- Added new functions `SIUnit::from_raw_parts` and `SIUnit::into_raw_parts`. [#55](https://github.com/itt-ustutt/quantity/pull/55)

## Fixed
- Fixed workflow for Apple universal2 wheels. [#55](https://github.com/itt-ustutt/quantity/pull/55)

## [0.6.1] - 2023-08-06
### Added
- Added new functions `Quantity::from_raw_parts` and `Quantity::into_raw_parts` for SI quantities. [#54](https://github.com/itt-ustutt/quantity/pull/54)

## [0.6.0] - 2023-01-20
### Added
- Added the new SI unit prefixes `QUECTO`, `RONTO`, `RONNA`, and `QUETTA`. [#49](https://github.com/itt-ustutt/quantity/pull/49)

### Packaging
- Updated `pyo3` and `numpy` dependencies to 0.18. [#52](https://github.com/itt-ustutt/quantity/pull/52)

## [0.5.1] - 2022-06-08
### Added
- Added implementations of `IntoIterator` for every `Quantity` with iteratable inner types. [#48](https://github.com/itt-ustutt/quantity/pull/48)
- Implemented `FromIterator` for `QuantityArray1`, enabling the use of `collect()` for quantity arrays. [#48](https://github.com/itt-ustutt/quantity/pull/48)

## [0.5.0] - 2022-03-09
### Packaging
- Updated `pyo3` and `numpy` dependencies to 0.16.
- Updated `ang` dependency to 0.6.

## [0.4.1] - 2022-01-10
### Added
- Added implementations of `__getstate__` and `__setstate__` functions for `PySINumber` and `PySIArrayX` to allow pickling of python objects.  [#43](https://github.com/itt-ustutt/quantity/pull/43)

### Fixed
- Formatted output for `SINumber`s shows the correct number of decimal places again. [#42](https://github.com/itt-ustutt/quantity/pull/42)
- Fixed representation of `SINumbers`s with zero value. [#42](https://github.com/itt-ustutt/quantity/pull/42)

## [0.4.0] - 2021-12-16
### Added
- `QuantityScalar1<U>::from_vec` as a simple way to convert vectors of scalar quantities to `QuantityArray`s. [#40](https://github.com/itt-ustutt/quantity/pull/40)
- Added constructor for `PySIArray1` that accepts `SINumber`, a list of `SINumber`s provided they have the same unit, and `PySIArray1` itself. [#40](https://github.com/itt-ustutt/quantity/pull/40)

### Removed
- Removed the `solve` function of `QuantityArray2` to avoid a dependency on external libraries (BLAS, LAPACK). [#41](https://github.com/itt-ustutt/quantity/pull/41)

### Packaging
- Updated `pyo3` and `numpy` dependencies to 0.15.

## [0.3.2] - 2021-11-18
### Added
- Added `J/kg/K` to the list of SI unit symbols. [#37](https://github.com/itt-ustutt/quantity/pull/37)
- Divide `SINumber`s and `SIArray`s by `CELSIUS`. Panics if the quantity is not a temperature. [#36](https://github.com/itt-ustutt/quantity/pull/36)
- Additional unit `DEBYE`. Can only be used together with SI units by squaring it first. [#38](https://github.com/itt-ustutt/quantity/pull/38)


## [0.3.1] - 2021-11-08
### Added
- Index into `PySIArray1` (get values, set values and iterate over all values). [#28](https://github.com/itt-ustutt/quantity/pull/28)
- Implement rich comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) in python. [#27](https://github.com/itt-ustutt/quantity/pull/27)
- Added additional arithmetic operations. [#26](https://github.com/itt-ustutt/quantity/pull/26)
  - `PySINumber` + `PySIArrayX`, `PySINumber` - `PySIArrayX`, `PySINumber` / `PySIArrayX`
  - `PySIArrayX` + `PySINumber`, `PySIArrayX` - `PySINumber`
  - `abs(PySINumber)`

### Packaging
- The standalone Python package is renamed to `si_units` to avoid a name conflict on PyPI and to better reflect its content. [#31](https://github.com/itt-ustutt/quantity/pull/31)

### Changed
- Comparisons (`PartialOrd` and `PartialEq`) of `Quantity`s now panic if they do not have the same unit. [#27](https://github.com/itt-ustutt/quantity/pull/27)
- Changed the order of the type inference in the arithmetic operations to check for scalars first. [#26](https://github.com/itt-ustutt/quantity/pull/26)

### Fixed
- Fix the formatting of `SINumber`s with negative values. [#24](https://github.com/itt-ustutt/quantity/pull/24)
- Correctly return a `np.ndarray` in multiplications of scalars with arrays in which the units cancel out. [#26](https://github.com/itt-ustutt/quantity/pull/26)

## [0.3.0] - 2021-08-13
### Added
- Added `PyAngle:From<Angle>` and `Angle:From<PyAngle>`. [#21](https://github.com/itt-ustutt/quantity/pull/21)
- Added `PySIArrayX:Deref<Target=SIArrayX>` to automatically convert between `&PySIArrayX` and `&SIArrayX`. [#21](https://github.com/itt-ustutt/quantity/pull/21)

### Changed
- Renamed module `pyquantity` to `python`. [#19](https://github.com/itt-ustutt/quantity/pull/19)
- Replaced all Python wrapper structs with tuple structs. [#19](https://github.com/itt-ustutt/quantity/pull/19)
- Made the fields of the tuple structs `pub(crate)`. [#21](https://github.com/itt-ustutt/quantity/pull/21)
- Made `PySINumber`, `PyAngle` and `PyCelsius` `Copy`. [#21](https://github.com/itt-ustutt/quantity/pull/21)
- Improved float and latex representations of some SI quantities. [#20](https://github.com/itt-ustutt/quantity/pull/19)

## [0.2.0] - 2021-07-26
### Packaging
- Updated `ndarray` dependency to 0.15.
- Updated `ang` dependency to 0.4.
- Updated `regex` dependency to 1.5.
- Updated `ndarray-linalg` dependency to 0.13.
- Updated `pyo3` dependency to 0.14.

### Added
- Constant `CELSIUS`, that can be used for simple temperature conversions, mirrored in python. [#17](https://github.com/itt-ustutt/quantity/pull/17)

### Changed
- Removed enclosing `$` from `to_latex` functions in rust. [#16](https://github.com/itt-ustutt/quantity/pull/16)
