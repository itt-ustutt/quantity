use super::*;
#[cfg(feature = "ndarray")]
use ndarray::{Array, Dimension};
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

const UNIT_SYMBOLS: [&str; 7] = ["s", "m", "kg", "A", "K", "mol", "cd"];

impl<
    Inner: fmt::Debug,
    const T: i8,
    const L: i8,
    const M: i8,
    const I: i8,
    const THETA: i8,
    const N: i8,
    const J: i8,
> fmt::Debug for Quantity<Inner, SIUnit<T, L, M, I, THETA, N, J>>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        let unit = [T, L, M, I, THETA, N, J]
            .iter()
            .zip(UNIT_SYMBOLS.iter())
            .filter_map(|(&u, &s)| match u {
                0 => None,
                1 => Some(s.to_owned()),
                _ => Some(format!("{s}^{u}")),
            })
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, " {unit}")
    }
}

impl<Inner: fmt::Display> fmt::Display for Quantity<Inner, _Dimensionless> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "pyo3")]
pub(crate) trait PrintUnit {
    const UNIT: &'static str;
}

macro_rules! impl_fmt {
    ($t:expr, $l:expr, $m:expr, $i:expr, $theta:expr, $n:expr, $unit:expr, $symbol:expr, $has_prefix:expr) => {
        impl<T> fmt::LowerExp for Quantity<T, SIUnit<$t, $l, $m, $i, $theta, $n, 0>>
        where
            for<'a> &'a T: Div<f64>,
            for<'a> Quot<&'a T, f64>: fmt::LowerExp,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (self / $unit).into_value().fmt(f)?;
                write!(f, " {}", $symbol)
            }
        }

        impl<T> fmt::UpperExp for Quantity<T, SIUnit<$t, $l, $m, $i, $theta, $n, 0>>
        where
            for<'a> &'a T: Div<f64>,
            for<'a> Quot<&'a T, f64>: fmt::UpperExp,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (self / $unit).into_value().fmt(f)?;
                write!(f, " {}", $symbol)
            }
        }

        #[cfg(feature = "ndarray")]
        impl<D: Dimension> fmt::Display
            for Quantity<Array<f64, D>, SIUnit<$t, $l, $m, $i, $theta, $n, 0>>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (self / $unit).into_value().fmt(f)?;
                write!(f, " {}", $symbol)
            }
        }

        impl fmt::Display for Quantity<f64, SIUnit<$t, $l, $m, $i, $theta, $n, 0>> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let (value, prefix) = get_prefix((self / $unit).into_value(), $has_prefix);
                if !((1e-2..1e4).contains(&value.abs()) || value == 0.0) {
                    write!(f, "{:e} {}{}", value, prefix, $symbol)
                } else {
                    value.fmt(f)?;
                    write!(f, " {}{}", prefix, $symbol)
                }
            }
        }

        #[cfg(feature = "python")]
        impl<T> PrintUnit for Quantity<T, SIUnit<$t, $l, $m, $i, $theta, $n, 0>> {
            const UNIT: &'static str = $symbol;
        }
    };
}

impl_fmt!(1, 0, 0, 0, 0, 0, SECOND, "s", Some(KILO));
impl_fmt!(0, 1, 0, 0, 0, 0, METER, "m", Some(MEGA));
impl_fmt!(0, 0, 1, 0, 0, 0, GRAM, "g", Some(MEGA));
impl_fmt!(0, 0, 0, 0, 0, 1, MOL, "mol", Some(MEGA));
impl_fmt!(0, 0, 0, 0, 1, 0, KELVIN, "K", None);
impl_fmt!(-1, 0, 0, 0, 0, 0, HERTZ, "Hz", Some(PETA));
impl_fmt!(-2, 1, 1, 0, 0, 0, NEWTON, "N", Some(PETA));
impl_fmt!(-2, -1, 1, 0, 0, 0, PASCAL, "Pa", Some(PETA));
impl_fmt!(-2, 2, 1, 0, 0, 0, JOULE, "J", Some(PETA));
impl_fmt!(-3, 2, 1, 0, 0, 0, WATT, "W", Some(PETA));
impl_fmt!(1, 0, 0, 1, 0, 0, COULOMB, "C", None);
impl_fmt!(-3, 2, 1, -1, 0, 0, VOLT, "V", Some(PETA));
impl_fmt!(4, -2, -1, 2, 0, 0, FARAD, "F", Some(PETA));
impl_fmt!(-3, 2, 1, -2, 0, 0, OHM, "Ω", Some(PETA));
impl_fmt!(3, -2, -1, 2, 0, 0, SIEMENS, "S", Some(PETA));
impl_fmt!(-2, 2, 1, -1, 0, 0, WEBER, "Wb", Some(PETA));
impl_fmt!(-2, 0, 1, -1, 0, 0, TESLA, "T", Some(PETA));
impl_fmt!(-2, 2, 1, -2, 0, 0, HENRY, "H", Some(PETA));

const M2: Area = Quantity(1.0, PhantomData);
const M3: Volume = Quantity(1.0, PhantomData);
const KG: Mass = KILOGRAM;
const JMK: MolarEntropy = Quantity(1.0, PhantomData);
const JKGK: SpecificEntropy = Quantity(1.0, PhantomData);
const WMK: ThermalConductivity = Quantity(1.0, PhantomData);
const GS: MassFlowRate = Quantity(1e-3, PhantomData);

impl_fmt!(0, -3, 0, 0, 0, 1, MOL / M3, "mol/m³", Some(MEGA));
impl_fmt!(0, -2, 0, 0, 0, 1, MOL / M2, "mol/m²", Some(MEGA));
impl_fmt!(0, -1, 0, 0, 0, 1, MOL / METER, "mol/m", Some(MEGA));
impl_fmt!(0, 3, 0, 0, 0, -1, M3 / MOL, "m³/mol", None);
impl_fmt!(0, 3, 0, 0, -1, -1, M3 / MOL / KELVIN, "m³/mol/K", None);
impl_fmt!(0, -3, 1, 0, 0, 0, GRAM / M3, "g/m³", Some(MEGA));
impl_fmt!(-2, 0, 1, 0, 0, 0, NEWTON / METER, "N/m", Some(PETA));
impl_fmt!(-1, 2, 1, 0, 0, 0, JOULE * SECOND, "J*s", Some(PETA));
impl_fmt!(-2, 2, 1, 0, 0, -1, JOULE / MOL, "J/mol", Some(PETA));
impl_fmt!(-2, 2, 1, 0, -1, 0, JOULE / KELVIN, "J/K", Some(PETA));
impl_fmt!(-2, 2, 1, 0, -1, -1, JMK, "J/mol/K", Some(PETA));
impl_fmt!(-2, 2, 0, 0, 0, 0, JOULE / KG, "J/kg", Some(PETA));
impl_fmt!(-2, 2, 0, 0, -1, 0, JKGK, "J/kg/K", Some(PETA));
impl_fmt!(-1, -1, 1, 0, 0, 0, PASCAL * SECOND, "Pa*s", Some(PETA));
impl_fmt!(-1, 1, 0, 0, 0, 0, METER / SECOND, "m/s", Some(MEGA));
impl_fmt!(-1, 2, 0, 0, 0, 0, M2 / SECOND, "m²/s", None);
impl_fmt!(-3, 1, 1, 0, -1, 0, WMK, "W/m/K", Some(PETA));
impl_fmt!(0, 0, 1, 0, 0, -1, GRAM / MOL, "g/mol", Some(MEGA));
impl_fmt!(0, 2, 0, 0, 0, 0, M2, "m²", None);
impl_fmt!(0, 3, 0, 0, 0, 0, M3, "m³", None);
impl_fmt!(-1, 3, -1, 0, 0, 0, M3 / KG / SECOND, "m³/kg/s²", None);
impl_fmt!(-3, 2, 1, 0, -1, 0, WATT / KELVIN, "W/K", None);
impl_fmt!(-3, 0, 1, 0, -1, 0, WMK / METER, "W/m²/K", None);
impl_fmt!(-3, 0, 1, 0, 0, 0, WATT / M2, "W/m²", None);
impl_fmt!(-1, 0, 1, 0, 0, 0, GS, "g/s", Some(MEGA));
impl_fmt!(-1, -2, 1, 0, 0, 0, GS / M2, "g/m²/s", Some(MEGA));

fn get_prefix(value: f64, has_prefix: Option<f64>) -> (f64, &'static str) {
    if let Some(p) = has_prefix {
        let abs_value = value.abs();
        let e: i8 = if abs_value > PICO && abs_value < p {
            (abs_value.log10().floor() as i8).div_euclid(3) * 3
        } else {
            0
        };
        let prefix = 10.0f64.powi(e as i32);
        return (value / prefix, PREFIX_SYMBOLS.get(&e).unwrap());
    }
    (value, "")
}

static PREFIX_SYMBOLS: LazyLock<HashMap<i8, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(0, " ");
    m.insert(-24, "y");
    m.insert(-21, "z");
    m.insert(-18, "a");
    m.insert(-15, "f");
    m.insert(-12, "p");
    m.insert(-9, "n");
    m.insert(-6, "µ");
    m.insert(-3, "m");
    m.insert(3, "k");
    m.insert(6, "M");
    m.insert(9, "G");
    m.insert(12, "T");
    m.insert(15, "P");
    m.insert(18, "E");
    m.insert(21, "Z");
    m.insert(24, "Y");
    m
});

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_degrees().fmt(f)?;
        write!(f, "°")
    }
}

impl<T: fmt::Debug> fmt::Debug for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        write!(f, " rad")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[cfg(feature = "ndarray")]
    use ndarray::arr1;

    #[test]
    fn test_fmt_si() {
        assert_eq!(format!("{RGAS:.3}"), "8.314  J/mol/K");
    }

    #[test]
    fn test_fmt_kg() {
        let m = 5.0 * KILOGRAM;
        let t = 4.0 * SECOND;
        let a = 0.5 * METER * METER;
        assert_eq!(format!("{:.3}", m / t), "1.250 kg/s");
        assert_eq!(format!("{:.3}", m / t / a), "2.500 kg/m²/s");
    }

    #[test]
    fn test_fmt_exp() {
        assert_eq!(format!("{:e}", PICO * METER), "1e-12 m");
        assert_eq!(format!("{:E}", 50.0 * KILO * GRAM), "5E4 g");
    }

    #[test]
    #[cfg(feature = "ndarray")]
    fn test_fmt_arr() {
        assert_eq!(
            format!("{}", arr1(&[273.15, 323.15]) * KELVIN),
            "[273.15, 323.15] K"
        );
        assert_eq!(format!("{:e}", arr1(&[3.0, 5.0]) * BAR), "[3e5, 5e5] Pa");
    }

    #[test]
    fn test_fmt_zero() {
        assert_eq!(format!("{}", 0.0 * KELVIN), "0 K");
        assert_eq!(format!("{:.2}", 0.0 * PASCAL), "0.00  Pa");
    }

    #[test]
    fn test_fmt_dimensionless() {
        assert_eq!(format!("{}", BAR / PASCAL), format!("{}", 1e5));
        assert_eq!(
            format!("{}", RGAS / (JOULE / (MOL * KELVIN))),
            format!("{}", 8.31446261815324)
        );
    }

    #[test]
    fn test_fmt_angle() {
        assert_eq!(format!("{}", 90.0 * DEGREES), "90°");
        assert_eq!(format!("{:.3?}", 45.0 * DEGREES), "0.785 rad");
        assert_eq!(format!("{:.2}", 0.5 * RADIANS), "28.65°");
    }
}
