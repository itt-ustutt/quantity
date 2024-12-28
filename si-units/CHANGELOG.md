# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.0] - 2024-12-08
### Removed
- Removed `SIArray1` class, which was just there for the namespace anyways. [#77](https://github.com/itt-ustutt/quantity/pull/77)

### Added
- Added `array()`, `linspace()` and `logspace()` as replacements for `SIArray1()`, `SIArray1.linspace()` and `SIArray1.logspace()`. [#77](https://github.com/itt-ustutt/quantity/pull/77)

## [0.10.0] - 2024-12-05
### Packaging
- Update `pyo3` and `numpy` dependencies to 0.23. [#76](https://github.com/itt-ustutt/quantity/pull/76)

## [0.9.1] - 2024-11-28
### Fixed
- Added division by `CELSIUS`. [#74](https://github.com/itt-ustutt/quantity/pull/74)

## [0.9.0] - 2024-10-24
### Changed
- Reimplemented `si-units` Python package independent of `quantity` crate in and more "pythonic" fashion. [#63](https://github.com/itt-ustutt/quantity/pull/63)