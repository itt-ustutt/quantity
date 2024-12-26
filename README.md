# quantity

[![crate](https://img.shields.io/crates/v/quantity.svg)](https://crates.io/crates/quantity)
[![documentation](https://docs.rs/quantity/badge.svg)](https://docs.rs/quantity)

Representation of quantities, i.e. of unit valued scalars and arrays.

The units are checked at compile time and can be arbitrarily complex. Additional to simple scalar quantities, it also provides utilities for vector valued quantities, based on the [ndarray](https://github.com/rust-ndarray/ndarray) crate, where all entries share the same unit.

The crate is published together with the standalone Python package [si-units](si-units). Together with [PyO3](https://github.com/PyO3/pyo3), the data types defined in the Rust crate can be directly used in a Python interface. See the [example](example/extend_quantity/README.md) for details.

## Installation and Usage

Add this to your `Cargo.toml`:

```
[dependencies]
quantity = "0.10"
```

## Examples

Calculate pressure of an ideal gas.

```rust
let temperature = 25.0 * CELSIUS;
let volume = 1.5 * METER.powi(3);
let moles = 75.0 * MOL;
let pressure = moles * RGAS * temperature / volume;
println!("{:.5}", pressure);            // 123.94785 kPa
```

Calculate the gravitational pull of the moon on the earth.

```rust
let mass_earth = 5.9724e24 * KILOGRAM;
let mass_moon = 7.346e22 * KILOGRAM;
let distance = 383.398 * KILO * METER;
let force = G * mass_earth * mass_moon / distance.powi(2);
println!("{:.5e}", force);              // 1.99208e26 N
```

Calculate the pressure distribution in the atmosphere using the barometric formula.

```rust
let z = SIArray1::linspace(1.0 * METER, 70.0 * KILO * METER, 10)?;
let g = 9.81 * METER / SECOND.powi(2);
let m = 28.949 * GRAM / MOL;
let t = 10.0 * CELSIUS;
let p0 = BAR;
let pressure = p0 * (-&z * m * g).to_reduced(RGAS * t)?.mapv(f64::exp);
for i in 0..10 {
    println!("z = {:8.5}   p = {:9.5}", z.get(i), pressure.get(i));
}
// z =  1.00000  m   p =  99.98794 kPa
// z =  7.77867 km   p =  39.12796 kPa
// z = 15.55633 km   p =  15.31182 kPa
// z = 23.33400 km   p =   5.99192 kPa
// z = 31.11167 km   p =   2.34480 kPa
// z = 38.88933 km   p = 917.58301  Pa
// z = 46.66700 km   p = 359.07479  Pa
// z = 54.44467 km   p = 140.51557  Pa
// z = 62.22233 km   p =  54.98750  Pa
// z = 70.00000 km   p =  21.51808  Pa
```

## Python Package

For information on how to install, use and build the Python package, see [here](si-units).

## Documentation

For the rust documentation, see [here](https://docs.rs/quantity).

For the python documentation, see [here](https://itt-ustutt.github.io/quantity/index.html).
