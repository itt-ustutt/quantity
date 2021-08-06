# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Renamed module `pyquantity` to `python`. [#19](https://github.com/itt-ustutt/quantity/pull/19)
- Replaced all Python wrapper structs with tuple structs. [#19](https://github.com/itt-ustutt/quantity/pull/19)

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