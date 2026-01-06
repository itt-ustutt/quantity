//! Representation of quantites, i.e. of unit valued scalars and arrays.
//!
//! The units are checked at compile time and can be arbitrarily complex. Additional to simple scalar quantities, it also provides utilities for vector valued quantities, based on the [ndarray] crate, where all entries share the same unit.
//!
//! For details on the available methods, see the documentation of the [Quantity] struct.
//!
//! ## SI Base Units and Associated Constants
//!
//! The module contains the seven SI base units and their associated, exact-valued constants.
//!
//! Unit | Unit symbol | Quantity | Associated constant | Associated constant name | Associated constant value
//! -|-|-|-|-|-
//! [SECOND] | $\text{s}$ | time | [DVCS] | Hyperfine transition frequency of Cs | $\Delta\\nu_\text{Cs}=9192631770\\,\text{Hz}$
//! [METER] | $\text{m}$ | length | [CLIGHT] | Speed of light | $c=299792458\\,\\frac{\text{m}}{\text{s}}$
//! [KILOGRAM] | $\text{kg}$ | mass | [PLANCK] | Planck constant | $h=6.62607015\times 10^{-34}\\,\text{J}\\cdot\text{s}$
//! [AMPERE] | $\text{A}$ | electric current | [QE] | Elementary charge | $e=1.602176634\\times 10^{-19}\\,\text{C}$
//! [KELVIN] | $\text{K}$ | thermodynamic temperature | [KB] | Boltzmann constant | $k_\text{B}=1.380649\times 10^{-23}\\,\\frac{\text{J}}{\text{K}}$
//! [MOL] | $\text{mol}$ | amount of substance | [NAV] | Avogadro constant | $N_\text{A}=6.02214076\times 10^{23}\\,\text{mol}^{-1}$
//! [CANDELA] | $\text{cd}$ | luminous intensity | [KCD] | Luminous efficacy of $540\\,\text{THz}$ radiation | $K_\text{cd}=683\\,\\frac{\text{lm}}{\text{W}}$
//!
//! ## Derived Units
//!
//! Additionally, it contains the following derived units.
//!
//! Unit | Unit symbol | Quantity | Definition
//! -|-|-|-
//! [HERTZ] | $\text{Hz}$ | frequency | $\text{s}^{-1}$
//! [NEWTON] | $\text{N}$ | force, weight | $\text{kg}\\frac{\text{m}}{\text{s}^2}$
//! [PASCAL] | $\text{Pa}$ | pressure, stress | $\\frac{\text{N}}{\text{m}^2}$
//! [JOULE] | $\text{J}$ | energy, work, heat | $\text{N}\text{m}$
//! [WATT] | $\text{W}$ | power, radiant flux | $\\frac{\text{J}}{\text{s}}$
//! [COULOMB] | $\text{C}$ | electric charge | $\text{A}\text{s}$
//! [VOLT] | $\text{V}$ | electrical potential difference | $\\frac{\text{W}}{\text{A}}$
//! [FARAD] | $\text{F}$ | capacitance | $\\frac{\text{C}}{\text{V}}$
//! [OHM] | $\text{Ω}$ | resistance, impedance, reactance | $\\frac{\text{V}}{\text{A}}$
//! [SIEMENS] | $\text{S}$ | electrical conductance | $\text{Ω}^{-1}$
//! [WEBER] | $\text{Wb}$ | magnetic flux | $\text{V}\text{s}$
//! [TESLA] | $\text{T}$ | magnetic flux density | $\\frac{\text{Wb}}{\text{m}^2}$
//! [HENRY] | $\text{H}$ | inductance | $\\frac{\text{Wb}}{\text{A}}$
//!
//! ## Additional units
//!
//! For convenience, a number of commonly used units that are not directly combinations of SI base units is also included.
//! These constants simplify the specification of properties, that are not given in SI units. However, as the representation
//! of quantities is unique, they do not appear in formatted outputs.
//!
//! Unit | Unit symbol | Quantity | Definition
//! -|-|-|-
//! [ANGSTROM] | $\text{\\AA}$ | length | $10^{-10}\\,\text{m}$
//! [AMU] | $\text{u}$ | mass | $1.6605390671738466\times 10^{-27}\\,\text{kg}$
//! [AU] | $\text{au}$ | length | $149597870700\\,\text{m}$
//! [BAR] | $\text{bar}$ | pressure | $10^5\\,\text{Pa}$
//! [CALORIE] | $\text{cal}$ | energy | $4.184\\,\text{J}$
//! [CELSIUS] | $^\\circ\text{C}$ | temperature | $t\\,^\\circ\text{C}=\\left(t+273.15\\right)\\,\text{K}$
//! [DAY] | $\text{d}$ | time | $86400\\,\text{s}$
//! [DEGREES] | $^\\circ$ | angle | $\\frac{\pi}{180}\\,\text{rad}$
//! [GRAM] | $\text{g}$ | mass | $10^{-3}\\,\text{kg}$
//! [HOUR] | $\text{h}$ | time | $3600\\,\text{s}$
//! [LITER] | $\text{l}$ | volume | $10^{-3}\\,\text{m}^3$
//! [MINUTE] | $\text{min}$ | time | $60\\,\text{s}$
//! [RADIANS] | $\text{rad}$ | angle |
//!
//! ## Additional constants
//!
//! Constant | Name | Symbol | Value
//! -|-|-|-
//! [G] | Gravitational constant | $G$ | $6.6743\\times 10^{-11}\\,\\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}$
//! [RGAS] | Ideal gas constant | $R=N_\text{Av}k_\text{B}$ | $8.31446261815324\\,\\frac{\text{J}}{\text{mol}\\cdot\text{K}}$
//!
//! ## Prefixes
//!
//! All units can be combined with the following prefixes:
//!
//! Prefix | Prefix symbol | value | | Prefix | Prefix symbol | value
//! -|-|-|-|-|-|-
//! [DECI] | $\text{d}$ | $10^{-1}$ | | [DECA] | $\text{da}$ | $10^{1}$
//! [CENTI] | $\text{c}$ | $10^{-2}$ | | [HECTO] | $\text{h}$ | $10^{2}$
//! [MILLI] | $\text{m}$ | $10^{-3}$ | | [KILO] | $\text{k}$ | $10^{3}$
//! [MICRO] | $\text{µ}$ | $10^{-6}$ | | [MEGA] | $\text{M}$ | $10^{6}$
//! [NANO] | $\text{n}$ | $10^{-9}$ | | [GIGA] | $\text{G}$ | $10^{9}$
//! [PICO] | $\text{p}$ | $10^{-12}$ | | [TERA] | $\text{T}$ | $10^{12}$
//! [FEMTO] | $\text{f}$ | $10^{-15}$ | | [PETA] | $\text{P}$ | $10^{15}$
//! [ATTO] | $\text{a}$ | $10^{-18}$ | | [EXA] | $\text{E}$ | $10^{18}$
//! [ZEPTO] | $\text{z}$ | $10^{-21}$ | | [ZETTA] | $\text{Z}$ | $10^{21}$
//! [YOCTO] | $\text{y}$ | $10^{-24}$ | | [YOTTA] | $\text{Y}$ | $10^{24}$
//! [RONTO] | $\text{r}$ | $10^{-27}$ | | [RONNA] | $\text{R}$ | $10^{27}$
//! [QUECTO] | $\text{q}$ | $10^{-30}$ | | [QUETTA] | $\text{Q}$ | $10^{30}$
//!
//! ## Examples
//!
//! Calculate pressure of an ideal gas.
//! ```
//! # use quantity::*;
//! let temperature = 25.0 * CELSIUS;
//! let volume = 1.5 * METER.powi::<3>();
//! let moles = 75.0 * MOL;
//! let pressure = moles * RGAS * temperature / volume;
//! println!("{:.5}", pressure);            // 123.94785 kPa
//! ```
//!
//! Calculate the gravitational pull of the moon on the earth.
//! ```
//! # use quantity::*;
//! let mass_earth = 5.9724e24 * KILOGRAM;
//! let mass_moon = 7.346e22 * KILOGRAM;
//! let distance = 383.398 * KILO * METER;
//! let force = G * mass_earth * mass_moon / distance.powi::<2>();
//! println!("{:.5e}", force);              // 1.99208e26 N
//! ```
//!
//! Calculate the pressure distribution in the atmosphere using the barometric formula.
//! Array operations require the `ndarray` feature.
//! ```
//! # #[cfg(feature = "ndarray")]
//! # {
//! # use quantity::*;
//! let z = Length::linspace(1.0 * METER, 70.0 * KILO * METER, 10);
//! let g = 9.81 * METER / SECOND.powi::<2>();
//! let m = 28.949 * GRAM / MOL;
//! let t = 10.0 * CELSIUS;
//! let p0 = BAR;
//! let pressure = (-z.clone() * m * g).convert_to(RGAS * t).mapv(f64::exp) * p0;
//! for i in 0..10 {
//!     println!("z = {:8.5}   p = {:9.5}", z.get(i), pressure.get(i));
//! }
//! // z =  1.00000  m   p =  99.98794 kPa
//! // z =  7.77867 km   p =  39.12796 kPa
//! // z = 15.55633 km   p =  15.31182 kPa
//! // z = 23.33400 km   p =   5.99192 kPa
//! // z = 31.11167 km   p =   2.34480 kPa
//! // z = 38.88933 km   p = 917.58301  Pa
//! // z = 46.66700 km   p = 359.07479  Pa
//! // z = 54.44467 km   p = 140.51557  Pa
//! // z = 62.22233 km   p =  54.98750  Pa
//! // z = 70.00000 km   p =  21.51808  Pa
//! # }
//! ```
//! ## Feature flags
//! Interoperability with other crates can be achieved by activating the following features:
#![doc = document_features::document_features!()]
#![warn(clippy::all)]
#[cfg(feature = "ndarray")]
use ndarray::{Array, ArrayBase, Data, Dimension};
use std::marker::PhantomData;
use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

#[cfg(feature = "num-dual")]
pub mod ad;
#[cfg(feature = "ndarray")]
mod array;
mod fmt;
#[cfg(feature = "nalgebra")]
mod nalgebra;
mod ops;
#[cfg(feature = "python")]
mod python;

type Sum<T1, T2> = <T1 as Add<T2>>::Output;
type Diff<T1, T2> = <T1 as Sub<T2>>::Output;
type Negate<T> = <T as Neg>::Output;
type Prod<T1, T2> = <T1 as Mul<T2>>::Output;
type Quot<T1, T2> = <T1 as Div<T2>>::Output;

pub struct Const<const N: i8>;

include!(concat!(env!("OUT_DIR"), "/const_impls.rs"));

#[derive(Clone, Copy)]
pub struct SIUnit<
    const T: i8,
    const L: i8,
    const M: i8,
    const I: i8,
    const THETA: i8,
    const N: i8,
    const J: i8,
>;

impl<
    const T1: i8,
    const L1: i8,
    const M1: i8,
    const I1: i8,
    const THETA1: i8,
    const N1: i8,
    const J1: i8,
    const T2: i8,
    const L2: i8,
    const M2: i8,
    const I2: i8,
    const THETA2: i8,
    const N2: i8,
    const J2: i8,
    const T3: i8,
    const L3: i8,
    const M3: i8,
    const I3: i8,
    const THETA3: i8,
    const N3: i8,
    const J3: i8,
> Add<SIUnit<T2, L2, M2, I2, THETA2, N2, J2>> for SIUnit<T1, L1, M1, I1, THETA1, N1, J1>
where
    Const<T1>: Add<Const<T2>, Output = Const<T3>>,
    Const<L1>: Add<Const<L2>, Output = Const<L3>>,
    Const<M1>: Add<Const<M2>, Output = Const<M3>>,
    Const<I1>: Add<Const<I2>, Output = Const<I3>>,
    Const<THETA1>: Add<Const<THETA2>, Output = Const<THETA3>>,
    Const<N1>: Add<Const<N2>, Output = Const<N3>>,
    Const<J1>: Add<Const<J2>, Output = Const<J3>>,
{
    type Output = SIUnit<T3, L3, M3, I3, THETA3, N3, J3>;

    fn add(self, _: SIUnit<T2, L2, M2, I2, THETA2, N2, J2>) -> Self::Output {
        SIUnit
    }
}

impl<
    const T1: i8,
    const L1: i8,
    const M1: i8,
    const I1: i8,
    const THETA1: i8,
    const N1: i8,
    const J1: i8,
    const T2: i8,
    const L2: i8,
    const M2: i8,
    const I2: i8,
    const THETA2: i8,
    const N2: i8,
    const J2: i8,
> Neg for SIUnit<T1, L1, M1, I1, THETA1, N1, J1>
where
    Const<T1>: Neg<Output = Const<T2>>,
    Const<L1>: Neg<Output = Const<L2>>,
    Const<M1>: Neg<Output = Const<M2>>,
    Const<I1>: Neg<Output = Const<I2>>,
    Const<THETA1>: Neg<Output = Const<THETA2>>,
    Const<N1>: Neg<Output = Const<N2>>,
    Const<J1>: Neg<Output = Const<J2>>,
{
    type Output = SIUnit<T2, L2, M2, I2, THETA2, N2, J2>;

    fn neg(self) -> Self::Output {
        SIUnit
    }
}

impl<
    const T1: i8,
    const L1: i8,
    const M1: i8,
    const I1: i8,
    const THETA1: i8,
    const N1: i8,
    const J1: i8,
    const T2: i8,
    const L2: i8,
    const M2: i8,
    const I2: i8,
    const THETA2: i8,
    const N2: i8,
    const J2: i8,
    const T3: i8,
    const L3: i8,
    const M3: i8,
    const I3: i8,
    const THETA3: i8,
    const N3: i8,
    const J3: i8,
> Sub<SIUnit<T2, L2, M2, I2, THETA2, N2, J2>> for SIUnit<T1, L1, M1, I1, THETA1, N1, J1>
where
    Const<T1>: Sub<Const<T2>, Output = Const<T3>>,
    Const<L1>: Sub<Const<L2>, Output = Const<L3>>,
    Const<M1>: Sub<Const<M2>, Output = Const<M3>>,
    Const<I1>: Sub<Const<I2>, Output = Const<I3>>,
    Const<THETA1>: Sub<Const<THETA2>, Output = Const<THETA3>>,
    Const<N1>: Sub<Const<N2>, Output = Const<N3>>,
    Const<J1>: Sub<Const<J2>, Output = Const<J3>>,
{
    type Output = SIUnit<T3, L3, M3, I3, THETA3, N3, J3>;

    fn sub(self, _: SIUnit<T2, L2, M2, I2, THETA2, N2, J2>) -> Self::Output {
        SIUnit
    }
}

impl<
    const T1: i8,
    const L1: i8,
    const M1: i8,
    const I1: i8,
    const THETA1: i8,
    const N1: i8,
    const J1: i8,
    const T2: i8,
    const L2: i8,
    const M2: i8,
    const I2: i8,
    const THETA2: i8,
    const N2: i8,
    const J2: i8,
    const E: i8,
> Mul<Const<E>> for SIUnit<T1, L1, M1, I1, THETA1, N1, J1>
where
    Const<T1>: Mul<Const<E>, Output = Const<T2>>,
    Const<L1>: Mul<Const<E>, Output = Const<L2>>,
    Const<M1>: Mul<Const<E>, Output = Const<M2>>,
    Const<I1>: Mul<Const<E>, Output = Const<I2>>,
    Const<THETA1>: Mul<Const<E>, Output = Const<THETA2>>,
    Const<N1>: Mul<Const<E>, Output = Const<N2>>,
    Const<J1>: Mul<Const<E>, Output = Const<J2>>,
{
    type Output = SIUnit<T2, L2, M2, I2, THETA2, N2, J2>;

    fn mul(self, _: Const<E>) -> Self::Output {
        SIUnit
    }
}

impl<
    const T1: i8,
    const L1: i8,
    const M1: i8,
    const I1: i8,
    const THETA1: i8,
    const N1: i8,
    const J1: i8,
    const T2: i8,
    const L2: i8,
    const M2: i8,
    const I2: i8,
    const THETA2: i8,
    const N2: i8,
    const J2: i8,
    const E: i8,
> Div<Const<E>> for SIUnit<T1, L1, M1, I1, THETA1, N1, J1>
where
    Const<T1>: Div<Const<E>, Output = Const<T2>>,
    Const<L1>: Div<Const<E>, Output = Const<L2>>,
    Const<M1>: Div<Const<E>, Output = Const<M2>>,
    Const<I1>: Div<Const<E>, Output = Const<I2>>,
    Const<THETA1>: Div<Const<E>, Output = Const<THETA2>>,
    Const<N1>: Div<Const<E>, Output = Const<N2>>,
    Const<J1>: Div<Const<E>, Output = Const<J2>>,
{
    type Output = SIUnit<T2, L2, M2, I2, THETA2, N2, J2>;

    fn div(self, _: Const<E>) -> Self::Output {
        SIUnit
    }
}

/// Physical quantity with compile-time checked unit.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Quantity<T, U>(T, PhantomData<U>);

pub type _Dimensionless = SIUnit<0, 0, 0, 0, 0, 0, 0>;
pub type _Time = SIUnit<1, 0, 0, 0, 0, 0, 0>;
pub type _Length = SIUnit<0, 1, 0, 0, 0, 0, 0>;
pub type _Mass = SIUnit<0, 0, 1, 0, 0, 0, 0>;
pub type _Current = SIUnit<0, 0, 0, 1, 0, 0, 0>;
pub type _Temperature = SIUnit<0, 0, 0, 0, 1, 0, 0>;
pub type _Moles = SIUnit<0, 0, 0, 0, 0, 1, 0>;
pub type _LuminousIntensity = SIUnit<0, 0, 0, 0, 0, 0, 1>;

pub type Dimensionless<T = f64> = Quantity<T, _Dimensionless>;
pub type Time<T = f64> = Quantity<T, _Time>;
pub type Length<T = f64> = Quantity<T, _Length>;
pub type Mass<T = f64> = Quantity<T, _Mass>;
pub type Current<T = f64> = Quantity<T, _Current>;
pub type Temperature<T = f64> = Quantity<T, _Temperature>;
pub type Moles<T = f64> = Quantity<T, _Moles>;
pub type LuminousIntensity<T = f64> = Quantity<T, _LuminousIntensity>;

pub type _Frequency = Negate<_Time>;
pub type Frequency<T = f64> = Quantity<T, _Frequency>;
pub type _Velocity = Diff<_Length, _Time>;
pub type Velocity<T = f64> = Quantity<T, _Velocity>;
pub type _Acceleration = Diff<_Velocity, _Time>;
pub type Acceleration<T = f64> = Quantity<T, _Acceleration>;
pub type _Force = Sum<_Mass, _Acceleration>;
pub type Force<T = f64> = Quantity<T, _Force>;
pub type _Area = Sum<_Length, _Length>;
pub type Area<T = f64> = Quantity<T, _Area>;
pub type _Volume = Sum<_Area, _Length>;
pub type Volume<T = f64> = Quantity<T, _Volume>;
pub type _Energy = Sum<_Force, _Length>;
pub type Energy<T = f64> = Quantity<T, _Energy>;
pub type _Pressure = Diff<_Energy, _Volume>;
pub type Pressure<T = f64> = Quantity<T, _Pressure>;
pub type _Power = Diff<_Energy, _Time>;
pub type Power<T = f64> = Quantity<T, _Power>;
pub type _Charge = Sum<_Current, _Time>;
pub type Charge<T = f64> = Quantity<T, _Charge>;
pub type _ElectricPotential = Diff<_Power, _Current>;
pub type ElectricPotential<T = f64> = Quantity<T, _ElectricPotential>;
pub type _Capacitance = Diff<_Charge, _ElectricPotential>;
pub type Capacitance<T = f64> = Quantity<T, _Capacitance>;
pub type _Resistance = Diff<_ElectricPotential, _Current>;
pub type Resistance<T = f64> = Quantity<T, _Resistance>;
pub type _ElectricalConductance = Negate<_Resistance>;
pub type ElectricalConductance<T = f64> = Quantity<T, _ElectricalConductance>;
pub type _MagneticFlux = Sum<_ElectricPotential, _Time>;
pub type MagneticFlux<T = f64> = Quantity<T, _MagneticFlux>;
pub type _MagneticFluxDensity = Diff<_MagneticFlux, _Area>;
pub type MagneticFluxDensity<T = f64> = Quantity<T, _MagneticFluxDensity>;
pub type _Inductance = Diff<_MagneticFlux, _Current>;
pub type Inductance<T = f64> = Quantity<T, _Inductance>;

pub type _Entropy = Diff<_Energy, _Temperature>;
pub type Entropy<T = f64> = Quantity<T, _Entropy>;
pub type _EntropyPerTemperature = Diff<_Entropy, _Temperature>;
pub type EntropyPerTemperature<T = f64> = Quantity<T, _EntropyPerTemperature>;
pub type _MolarEntropy = Diff<_Entropy, _Moles>;
pub type MolarEntropy<T = f64> = Quantity<T, _MolarEntropy>;
pub type _MolarEnergy = Diff<_Energy, _Moles>;
pub type MolarEnergy<T = f64> = Quantity<T, _MolarEnergy>;
pub type _SpecificEntropy = Diff<_Entropy, _Mass>;
pub type SpecificEntropy<T = f64> = Quantity<T, _SpecificEntropy>;
pub type _SpecificEnergy = Diff<_Energy, _Mass>;
pub type SpecificEnergy<T = f64> = Quantity<T, _SpecificEnergy>;
pub type _MolarWeight = Diff<_Mass, _Moles>;
pub type MolarWeight<T = f64> = Quantity<T, _MolarWeight>;
pub type _Density = Diff<_Moles, _Volume>;
pub type Density<T = f64> = Quantity<T, _Density>;
pub type _MassDensity = Diff<_Mass, _Volume>;
pub type MassDensity<T = f64> = Quantity<T, _MassDensity>;
pub type _PressurePerVolume = Diff<_Pressure, _Volume>;
pub type PressurePerVolume<T = f64> = Quantity<T, _PressurePerVolume>;
pub type _PressurePerTemperature = Diff<_Pressure, _Temperature>;
pub type PressurePerTemperature<T = f64> = Quantity<T, _PressurePerTemperature>;
pub type _Compressibility = Negate<_Pressure>;
pub type Compressibility<T = f64> = Quantity<T, _Compressibility>;
pub type _MolarVolume = Diff<_Volume, _Moles>;
pub type MolarVolume<T = f64> = Quantity<T, _MolarVolume>;
pub type _EntropyDensity = Diff<_Entropy, _Volume>;
pub type EntropyDensity<T = f64> = Quantity<T, _EntropyDensity>;
pub type _Action = Sum<_Energy, _Time>;
pub type Action<T = f64> = Quantity<T, _Action>;
pub type _HeatCapacityRate = Diff<_Power, _Temperature>;
pub type HeatCapacityRate<T = f64> = Quantity<T, _HeatCapacityRate>;
pub type _MassFlowRate = Diff<_Mass, _Time>;
pub type MassFlowRate<T = f64> = Quantity<T, _MassFlowRate>;
pub type _MoleFlowRate = Diff<_Moles, _Time>;
pub type MoleFlowRate<T = f64> = Quantity<T, _MoleFlowRate>;
pub type _MassFlux = Diff<_MassFlowRate, _Area>;
pub type MassFlux<T = f64> = Quantity<T, _MassFlux>;
pub type _HeatFlux = Diff<_Power, _Area>;
pub type HeatFlux<T = f64> = Quantity<T, _HeatFlux>;

pub type _Viscosity = Sum<_Pressure, _Time>;
pub type Viscosity<T = f64> = Quantity<T, _Viscosity>;
pub type _Diffusivity = Sum<_Velocity, _Length>;
pub type Diffusivity<T = f64> = Quantity<T, _Diffusivity>;
pub type _ThermalConductivity = Diff<_Power, Sum<_Length, _Temperature>>;
pub type ThermalConductivity<T = f64> = Quantity<T, _ThermalConductivity>;
pub type _ThermalTransmittance = Diff<_Power, Sum<_Area, _Temperature>>;
pub type ThermalTransmittance<T = f64> = Quantity<T, _ThermalTransmittance>;
pub type _ThermalResistance = Negate<_ThermalTransmittance>;
pub type ThermalResistance<T = f64> = Quantity<T, _ThermalResistance>;
pub type _SurfaceTension = Diff<_Force, _Length>;
pub type SurfaceTension<T = f64> = Quantity<T, _SurfaceTension>;

/// SI base unit second $\\left(\text{s}\\right)$
pub const SECOND: Time = Quantity(1.0, PhantomData);
/// SI base unit meter $\\left(\text{m}\\right)$
pub const METER: Length = Quantity(1.0, PhantomData);
/// SI base unit kilogram $\\left(\text{kg}\\right)$
pub const KILOGRAM: Mass = Quantity(1.0, PhantomData);
/// SI base unit Ampere $\\left(\text{A}\\right)$
pub const AMPERE: Current = Quantity(1.0, PhantomData);
/// SI base unit Kelvin $\\left(\text{K}\\right)$
pub const KELVIN: Temperature = Quantity(1.0, PhantomData);
/// SI base unit mol $\\left(\text{mol}\\right)$
pub const MOL: Moles = Quantity(1.0, PhantomData);
/// SI base unit candela $\\left(\text{cd}\\right)$
pub const CANDELA: LuminousIntensity = Quantity(1.0, PhantomData);

/// Derived unit Hertz $\\left(1\\,\text{Hz}=1\\,\text{s}^{-1}\\right)$
pub const HERTZ: Frequency = Quantity(1.0, PhantomData);
/// Derived unit Newton $\\left(1\\,\text{N}=1\\,\text{kg}\\frac{\text{m}}{\text{s}^2}\\right)$
pub const NEWTON: Force = Quantity(1.0, PhantomData);
/// Derived unit Pascal $\\left(1\\,\text{Pa}=1\\,\\frac{\text{kg}}{\text{m}\\cdot\text{s}^2}\\right)$
pub const PASCAL: Pressure = Quantity(1.0, PhantomData);
/// Derived unit Joule $\\left(1\\,\text{J}=1\\,\text{kg}\\frac{\text{m}^2}{\text{s}^2}\\right)$
pub const JOULE: Energy = Quantity(1.0, PhantomData);
/// Derived unit Watt $\\left(1\\,\text{J}=1\\,\text{kg}\\frac{\text{m}^2}{\text{s}^3}\\right)$
pub const WATT: Power = Quantity(1.0, PhantomData);
/// Derived unit Coulomb $\\left(1\\,\text{C}=1\\,\text{A}\cdot\text{s}\\right)$
pub const COULOMB: Charge = Quantity(1.0, PhantomData);
/// Derived unit Volt $\\left(1\\,\text{V}=1\\,\\frac{\text{W}}{\text{A}}\\right)$
pub const VOLT: ElectricPotential = Quantity(1.0, PhantomData);
/// Derived unit Farad $\\left(1\\,\text{F}=1\\,\\frac{\text{C}}{\text{V}}\\right)$
pub const FARAD: Capacitance = Quantity(1.0, PhantomData);
/// Derived unit Ohm $\\left(1\\,\text{Ω}=1\\,\\frac{\text{V}}{\text{A}}\\right)$
pub const OHM: Resistance = Quantity(1.0, PhantomData);
/// Derived unit Siemens $\\left(1\\,\text{S}=1\\,\text{Ω}^{-1}\\right)$
pub const SIEMENS: ElectricalConductance = Quantity(1.0, PhantomData);
/// Derived unit Weber $\\left(1\\,\text{Wb}=1\\,\text{V}\\cdot\text{s}\\right)$
pub const WEBER: MagneticFlux = Quantity(1.0, PhantomData);
/// Derived unit Tesla $\\left(1\\,\text{T}=1\\,\\frac{\text{Wb}}{\text{m}^2}\\right)$
pub const TESLA: MagneticFluxDensity = Quantity(1.0, PhantomData);
/// Derived unit Henry $\\left(1\\,\text{T}=1\\,\\frac{\text{Wb}}{\text{A}}\\right)$
pub const HENRY: Inductance = Quantity(1.0, PhantomData);

/// Additional unit Ångstrom $\\left(1\\,\text{\\AA}=10^{-10}\\,\text{m}\\right)$
pub const ANGSTROM: Length = Quantity(1e-10, PhantomData);
/// Additional unit unified atomic mass $\\left(1\\,\text{u}\\approx 1.660539\\times 10^{-27}\\,\text{kg}\\right)$
pub const AMU: Mass = Quantity(1.6605390671738466e-27, PhantomData);
/// Additional unit astronomical unit $\\left(1\\,\text{au}=149597870700\\,\text{m}\\right)$
pub const AU: Length = Quantity(149597870700.0, PhantomData);
/// Additional unit bar $\\left(1\\,\text{bar}=10^5\\,\text{Pa}\\right)$
pub const BAR: Pressure = Quantity(1e5, PhantomData);
/// Additional unit calorie $\\left(1\\,\text{cal}=4.184\\,\text{J}\\right)$
pub const CALORIE: Energy = Quantity(4.184, PhantomData);
/// Additional unit day $\\left(1\\,\text{d}=86400,\text{s}\\right)$
pub const DAY: Time = Quantity(86400.0, PhantomData);
/// Additional unit gram $\\left(1\\,\text{g}=10^{-3}\\,\text{kg}\\right)$
pub const GRAM: Mass = Quantity(1e-3, PhantomData);
/// Additional unit hour $\\left(1\\,\text{h}=3600,\text{s}\\right)$
pub const HOUR: Time = Quantity(3600.0, PhantomData);
/// Additional unit liter $\\left(1\\,\text{l}=10^{-3}\\,\text{m}^3\\right)$
pub const LITER: Volume = Quantity(1e-3, PhantomData);
/// Additional unit minute $\\left(1\\,\text{min}=60,\text{s}\\right)$
pub const MINUTE: Time = Quantity(60.0, PhantomData);

/// Boltzmann constant $\\left(k_\text{B}=1.380649\times 10^{-23}\\,\\frac{\text{J}}{\text{K}}\\right)$
pub const KB: Entropy = Quantity(1.380649e-23, PhantomData);
/// Avogadro constant $\\left(N_\text{A}=6.02214076\times 10^{23}\\,\text{mol}^{-1}\\right)$
pub const NAV: Quantity<f64, Negate<_Moles>> = Quantity(6.02214076e23, PhantomData);
/// Planck constant $\\left(h=6.62607015\times 10^{-34}\\,\text{J}\\cdot\text{s}\\right)$
pub const PLANCK: Action = Quantity(6.62607015e-34, PhantomData);
/// Ideal gas constant $\\left(R=8.31446261815324\\,\\frac{\text{J}}{\text{molK}}\\right)$
pub const RGAS: MolarEntropy = Quantity(8.31446261815324, PhantomData);
/// Hyperfine transition frequency of Cs $\\left(\Delta\\nu_\text{Cs}=9192631770\\,\text{Hz}\\right)$
pub const DVCS: Frequency = Quantity(9192631770.0, PhantomData);
/// Elementary charge $\\left(e=1.602176634\\times 10^{-19}\\,\text{C}\\right)$
pub const QE: Charge = Quantity(1.602176634e-19, PhantomData);
/// Speed of light $\\left(c=299792458\\,\\frac{\text{m}}{\text{s}}\\right)$
pub const CLIGHT: Velocity = Quantity(299792458.0, PhantomData);
/// Luminous efficacy of $540\\,\text{THz}$ radiation $\\left(K_\text{cd}=683\\,\\frac{\text{lm}}{\text{W}}\\right)$
pub const KCD: Quantity<f64, SIUnit<-2, -1, 3, 0, 0, 0, 1>> = Quantity(683.0, PhantomData);
/// Gravitational constant $\\left(G=6.6743\\times 10^{-11}\\,\\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}\\right)$
pub const G: Quantity<f64, SIUnit<-2, 3, -1, 0, 0, 0, 0>> = Quantity(6.6743e-11, PhantomData);

/// Prefix quecto $\\left(\text{q}=10^{-30}\\right)$
pub const QUECTO: f64 = 1e-30;
/// Prefix ronto $\\left(\text{r}=10^{-27}\\right)$
pub const RONTO: f64 = 1e-27;
/// Prefix yocto $\\left(\text{y}=10^{-24}\\right)$
pub const YOCTO: f64 = 1e-24;
/// Prefix zepto $\\left(\text{z}=10^{-21}\\right)$
pub const ZEPTO: f64 = 1e-21;
/// Prefix atto $\\left(\text{a}=10^{-18}\\right)$
pub const ATTO: f64 = 1e-18;
/// Prefix femto $\\left(\text{f}=10^{-15}\\right)$
pub const FEMTO: f64 = 1e-15;
/// Prefix pico $\\left(\text{p}=10^{-12}\\right)$
pub const PICO: f64 = 1e-12;
/// Prefix nano $\\left(\text{n}=10^{-9}\\right)$
pub const NANO: f64 = 1e-9;
/// Prefix micro $\\left(\text{µ}=10^{-6}\\right)$
pub const MICRO: f64 = 1e-6;
/// Prefix milli $\\left(\text{m}=10^{-3}\\right)$
pub const MILLI: f64 = 1e-3;
/// Prefix centi $\\left(\text{c}=10^{-2}\\right)$
pub const CENTI: f64 = 1e-2;
/// Prefix deci $\\left(\text{d}=10^{-1}\\right)$
pub const DECI: f64 = 1e-1;
/// Prefix deca $\\left(\text{da}=10^{1}\\right)$
pub const DECA: f64 = 1e1;
/// Prefix hecto $\\left(\text{h}=10^{2}\\right)$
pub const HECTO: f64 = 1e2;
/// Prefix kilo $\\left(\text{k}=10^{3}\\right)$
pub const KILO: f64 = 1e3;
/// Prefix mega $\\left(\text{M}=10^{6}\\right)$
pub const MEGA: f64 = 1e6;
/// Prefix giga $\\left(\text{G}=10^{9}\\right)$
pub const GIGA: f64 = 1e9;
/// Prefix tera $\\left(\text{T}=10^{12}\\right)$
pub const TERA: f64 = 1e12;
/// Prefix peta $\\left(\text{P}=10^{15}\\right)$
pub const PETA: f64 = 1e15;
/// Prefix exa $\\left(\text{E}=10^{18}\\right)$
pub const EXA: f64 = 1e18;
/// Prefix zetta $\\left(\text{Z}=10^{21}\\right)$
pub const ZETTA: f64 = 1e21;
/// Prefix yotta $\\left(\text{Y}=10^{24}\\right)$
pub const YOTTA: f64 = 1e24;
/// Prefix ronna $\\left(\text{R}=10^{27}\\right)$
pub const RONNA: f64 = 1e27;
/// Prefix quetta $\\left(\text{Q}=10^{30}\\right)$
pub const QUETTA: f64 = 1e30;

/// Additional unit degrees Celsius
pub struct CELSIUS;

impl Mul<CELSIUS> for f64 {
    type Output = Temperature<f64>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, _: CELSIUS) -> Temperature<f64> {
        Quantity(self + 273.15, PhantomData)
    }
}

#[cfg(feature = "ndarray")]
impl<S: Data<Elem = f64>, D: Dimension> Mul<CELSIUS> for ArrayBase<S, D> {
    type Output = Temperature<Array<f64, D>>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, _: CELSIUS) -> Temperature<Array<f64, D>> {
        Quantity(&self + 273.15, PhantomData)
    }
}

impl Div<CELSIUS> for Temperature<f64> {
    type Output = f64;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, _: CELSIUS) -> Self::Output {
        self.0 - 273.15
    }
}

#[cfg(feature = "ndarray")]
impl<D: Dimension> Div<CELSIUS> for Temperature<Array<f64, D>> {
    type Output = Array<f64, D>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, _: CELSIUS) -> Self::Output {
        self.0 - 273.15
    }
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Radians;
pub type Angle<T = f64> = Quantity<T, Radians>;

/// Angle unit radians $\\left(\text{rad}\\right)$
pub const RADIANS: Angle = Quantity(1.0, PhantomData);
/// Angle unit degrees $\\left(1°=\\frac{\\pi}{180}\text{rad}\\right)$
pub const DEGREES: Angle = Quantity(std::f64::consts::PI / 180., PhantomData);

impl Angle {
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    pub fn tan(self) -> f64 {
        self.0.tan()
    }

    pub fn asin(x: f64) -> Self {
        Quantity(x.asin(), PhantomData)
    }

    pub fn acos(x: f64) -> Self {
        Quantity(x.acos(), PhantomData)
    }

    pub fn atan(x: f64) -> Self {
        Quantity(x.atan(), PhantomData)
    }

    pub fn atan2(y: f64, x: f64) -> Self {
        Quantity(y.atan2(x), PhantomData)
    }
}

impl<T> Dimensionless<T> {
    /// Return the value of a dimensionless quantity.
    pub fn into_value(self) -> T {
        self.0
    }
}

impl<T, U> Quantity<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }

    /// Convert a quantity into the given unit and return it
    /// as a float or array.
    pub fn convert_to<T2>(&self, unit: Quantity<T2, U>) -> Quot<&T, T2>
    where
        for<'a> &'a T: Div<T2>,
    {
        &self.0 / unit.0
    }

    /// Convert a quantity into the given unit and return it
    /// as a float or array.
    pub fn convert_into<T2>(self, unit: Quantity<T2, U>) -> Quot<T, T2>
    where
        T: Div<T2>,
    {
        self.0 / unit.0
    }
}

impl<T> Deref for Dimensionless<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quantity_instantiation() {
        let t = 10.0 * SECOND;
        assert_eq!(t.0, 10.0);

        let l = 5.0 * METER;
        assert_eq!(l.0, 5.0);
    }

    #[test]
    fn test_quantity_conversion() {
        let dist = 1.5 * KILO * METER;
        let raw_m = dist.convert_into(METER);
        assert!((raw_m - 1500.0).abs() < 1e-10);

        let km = Quantity::new(1000.0);
        let val_km = dist.convert_to(km);
        assert!((val_km - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_conversion() {
        let c = 0.0 * CELSIUS;
        assert_eq!(c.0, 273.15);

        let zero = c / CELSIUS;
        assert!(zero.abs() < 1e-15);
    }

    #[test]
    fn test_prefix_scaling() {
        let v1 = 1.0 * MILLI * METER;
        let v2 = 1.0 * KILO * METER;

        assert_eq!(v1.0, 0.001);
        assert_eq!(v2.0, 1000.0);
    }

    #[test]
    fn test_quantity_arithmetic() {
        let d = 10.0 * METER;
        let t = 2.0 * SECOND;

        let v = d / t;
        assert_eq!(v.0, 5.0);

        let m = 5.0 * KILOGRAM;
        let a = 2.0 * METER / (SECOND * SECOND);
        let f = m * a;
        assert_eq!(f.0, 10.0);

        let l1 = 1.0 * METER;
        let l2 = 2.0 * METER;
        let l3 = l1 + l2;
        assert_eq!(l3.0, 3.0);
    }

    #[test]
    fn test_angles() {
        let ninety_deg = 90.0 * DEGREES;
        let half_pi = std::f64::consts::FRAC_PI_2;

        assert!((ninety_deg.0 - half_pi).abs() < 1e-10);
        assert!((ninety_deg.sin() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_deref() {
        let pressure = 1.0135 * BAR;
        let x = (pressure / PASCAL).ln();
        assert_eq!(x, 1.0135e5_f64.ln())
    }
}
