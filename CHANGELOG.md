# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
<<<<<<< HEAD
- Added `J/kg/K` to the list of SI unit symbols. [#37](https://github.com/itt-ustutt/quantity/pull/37)
- Divide `SINumber`s and `SIArray`s by `CELSIUS`. Panics if the quantity is not a temperature. [#36](https://github.com/itt-ustutt/quantity/pull/36)
=======
- Additional unit `DEBYE`. Can only be used together with SI units by squaring it first. [#38](https://github.com/itt-ustutt/quantity/pull/38)
>>>>>>> a991a8b (Added new additional unit DEBYE)

## [0.3.2] - 2021-11-18

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
