use super::si::*;
use ndarray::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

macro_rules! impl_fmt {
    ($trt:path) => {
        impl $trt for SINumber {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if DERIVED_UNITS.contains_key(&self.unit) && !self.is_nan() {
                    let (unit, symbol, _, _, _) = DERIVED_UNITS.get(&self.unit).unwrap();
                    let value = self.to_reduced(*unit).unwrap();
                    value.fmt(f)?;
                    write!(f, " {}", symbol)
                } else {
                    self.value.fmt(f)?;
                    write!(f, " {}", self.unit)
                }
            }
        }
    };
}
impl_fmt!(fmt::LowerExp);
impl_fmt!(fmt::UpperExp);

impl fmt::Display for SINumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if DERIVED_UNITS.contains_key(&self.unit) && !self.is_nan() {
            let (unit, symbol, has_prefix, _, _) = DERIVED_UNITS.get(&self.unit).unwrap();
            let (value, prefix) = get_prefix(self.to_reduced(*unit).unwrap(), *has_prefix);
            if !((1e-2..1e4).contains(&value.abs()) || value == 0.0) {
                write!(f, "{:e} {}{}", value, prefix, symbol)
            } else {
                value.fmt(f)?;
                write!(f, " {}{}", prefix, symbol)
            }
        } else if !((1e-2..1e4).contains(&self.value.abs()) || self.value == 0.0) {
            write!(f, "{:e} {}", self.value, self.unit)
        } else {
            self.value.fmt(f)?;
            write!(f, " {}", self.unit)
        }
    }
}

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

impl SINumber {
    pub fn to_latex(&self) -> String {
        if DERIVED_UNITS.contains_key(&self.unit) && !self.is_nan() {
            let (unit, _, has_prefix, symbols, exponents) = DERIVED_UNITS.get(&self.unit).unwrap();
            let (value, prefix) = get_prefix(self.to_reduced(*unit).unwrap(), *has_prefix);
            format!(
                "{}\\,{}",
                float_to_latex(value),
                &unit_to_latex(symbols, exponents, Some(prefix))
            )
        } else {
            format!("{}\\,{}", float_to_latex(self.value), &self.unit.to_latex())
        }
    }
}

fn float_to_latex(f: f64) -> String {
    if f == 0.0 {
        return "0".to_string();
    }
    let e = f.abs().log10().floor() as i32;
    match e {
        -1 => trim_zeros(format!("{:.5}", f)),
        0 => trim_zeros(format!("{:.4}", f)),
        1 => trim_zeros(format!("{:.3}", f)),
        2 => trim_zeros(format!("{:.2}", f)),
        3 => trim_zeros(format!("{:.1}", f)),
        _ => format!(
            "{}\\times10^{{{}}}",
            trim_zeros(format!("{:.4}", f / 10.0f64.powi(e))),
            e
        ),
    }
}

fn trim_zeros(x: String) -> String {
    let mut l = x.len();
    for c in x.chars().rev() {
        match c {
            '0' => l -= 1,
            '.' => {
                l -= 1;
                break;
            }
            _ => break,
        }
    }
    if l == 0 {
        "0".to_string()
    } else {
        x[..l].to_string()
    }
}

impl fmt::Display for SIUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match DERIVED_UNITS.get(self) {
            Some((_, symbol, _, _, _)) => {
                write!(f, "{}", symbol)
            }
            None => {
                let st = self
                    .0
                    .iter()
                    .zip(UNIT_SYMBOLS.iter())
                    .filter_map(|(&u, &s)| match u {
                        0 => None,
                        1 => Some(s.to_owned()),
                        _ => Some(format!("{}^{}", s, u)),
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "{}", st)
            }
        }
    }
}

impl SIUnit {
    pub fn to_latex(&self) -> String {
        match DERIVED_UNITS.get(self) {
            Some((_, _, _, symbols, exponents)) => unit_to_latex(symbols, exponents, None),
            None => unit_to_latex(&UNIT_SYMBOLS, &self.0, None),
        }
    }
}

fn unit_to_latex(symbols: &[&str], exponents: &[i8], prefix: Option<&str>) -> String {
    let mut num = Vec::with_capacity(7);
    let mut den = Vec::with_capacity(7);
    let prefix_exp = prefix.map(|_| 1);
    for (&s, &e) in prefix
        .iter()
        .chain(symbols.iter())
        .zip(prefix_exp.iter().chain(exponents.iter()))
    {
        match e.cmp(&0) {
            Ordering::Greater => num.push((s, e)),
            Ordering::Less => den.push((s, -e)),
            Ordering::Equal => {}
        }
    }
    let num_st = unit_to_latex_product(num);
    let den_st = unit_to_latex_product(den);
    match (num_st, den_st) {
        (None, None) => String::new(),
        (Some(num), None) => format!("\\mathrm{{{}}}", num),
        (None, Some(den)) => format!("\\mathrm{{\\frac{{1}}{{{}}}}}", den),
        (Some(num), Some(den)) => format!("\\mathrm{{\\frac{{{}}}{{{}}}}}", num, den),
    }
}

fn unit_to_latex_product(vec: Vec<(&str, i8)>) -> Option<String> {
    match vec.len() {
        0 => None,
        _ => Some(
            vec.into_iter()
                .map(|(s, e)| match e {
                    1 => s.to_string(),
                    _ => format!("{}^{{{}}}", s, e),
                })
                .collect::<Vec<String>>()
                .join(""),
        ),
    }
}

impl<D: Dimension> fmt::Display for SIArray<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match DERIVED_UNITS.get(&self.unit) {
            Some((unit, symbol, _, _, _)) => {
                write!(f, "{} {}", self.to_reduced(*unit).unwrap(), symbol)
            }
            None => {
                write!(f, "{} {}", self.value, self.unit)
            }
        }
    }
}

impl fmt::Display for Debye {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} De", self.0)
    }
}

impl Debye {
    pub fn to_latex(&self) -> String {
        format!("{}\\,\\mathrm{{De}}", float_to_latex(self.0))
    }
}

const UNIT_SYMBOLS: [&str; 7] = ["m", "kg", "s", "A", "mol", "K", "cd"];

static DERIVED_UNIT_SYMBOLS: LazyLock<HashMap<&'static str, (SINumber, Option<f64>)>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert("m", (METER, Some(MEGA)));
        m.insert("g", (GRAM, Some(MEGA)));
        m.insert("s", (SECOND, Some(KILO)));
        m.insert("mol", (MOL, Some(MEGA)));
        m.insert("K", (KELVIN, None));
        m.insert("Hz", (HERTZ, Some(PETA)));
        m.insert("N", (NEWTON, Some(PETA)));
        m.insert("Pa", (PASCAL, Some(PETA)));
        m.insert("J", (JOULE, Some(PETA)));
        m.insert("W", (WATT, Some(PETA)));
        m.insert("m³", (METER.powi(3), None));
        m.insert("m²", (METER.powi(2), None));
        m.insert("kg", (KILOGRAM, None));
        m.insert("C", (COULOMB, None));
        m.insert("V", (VOLT, Some(PETA)));
        m.insert("F", (FARAD, Some(PETA)));
        m.insert("Ω", (OHM, Some(PETA)));
        m.insert("S", (SIEMENS, Some(PETA)));
        m.insert("Wb", (WEBER, Some(PETA)));
        m.insert("T", (TESLA, Some(PETA)));
        m.insert("H", (HENRY, Some(PETA)));
        m.insert("lm", (CANDELA, None));
        m.insert("s²", (SECOND.powi(2), None));
        m
    });

type SIUnitSymbol = (SINumber, String, Option<f64>, Vec<&'static str>, Vec<i8>);

fn insert_derived_unit(map: &mut HashMap<SIUnit, SIUnitSymbol>, s: &'static str) {
    let u_reg = Regex::new("([\\*/])").unwrap();
    let o_reg = Regex::new("mol|m³|m²|m|g|kg|s²|s|K|Hz|N|Pa|J|Wb|W|C|V|F|Ω|S|T|H|lm").unwrap();
    let mut unit = None;
    let mut has_prefix = None;
    let mut symbols = Vec::new();
    let mut exponents = Vec::new();
    for (o, u) in o_reg.split(&format!("*{}", s)).zip(u_reg.split(s)) {
        let (si, hp) = *DERIVED_UNIT_SYMBOLS.get(u).unwrap();
        if unit.is_none() {
            has_prefix = hp;
        }
        match o {
            "*" => unit = Some(unit.map_or(si, |unit| unit * si)),
            "/" => unit = Some(unit.map_or(si, |unit| unit / si)),
            _ => {}
        }
        symbols.push(match u {
            "m³" | "m²" => "m",
            _ => u,
        });
        exponents.push(
            match u {
                "m³" => 3,
                "m²" => 2,
                _ => 1,
            } * match o {
                "*" => 1,
                "/" => -1,
                _ => 0,
            },
        );
    }
    let unit = unit.unwrap();
    map.insert(
        unit.unit,
        (unit, s.replace('*', ""), has_prefix, symbols, exponents),
    );
}

static DERIVED_UNITS: LazyLock<HashMap<SIUnit, SIUnitSymbol>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    insert_derived_unit(&mut m, "m");
    insert_derived_unit(&mut m, "g");
    insert_derived_unit(&mut m, "s");
    insert_derived_unit(&mut m, "mol");
    insert_derived_unit(&mut m, "K");
    insert_derived_unit(&mut m, "Hz");
    insert_derived_unit(&mut m, "N");
    insert_derived_unit(&mut m, "Pa");
    insert_derived_unit(&mut m, "J");
    insert_derived_unit(&mut m, "W");
    insert_derived_unit(&mut m, "C");
    insert_derived_unit(&mut m, "V");
    insert_derived_unit(&mut m, "F");
    insert_derived_unit(&mut m, "Ω");
    insert_derived_unit(&mut m, "S");
    insert_derived_unit(&mut m, "Wb");
    insert_derived_unit(&mut m, "T");
    insert_derived_unit(&mut m, "H");
    insert_derived_unit(&mut m, "mol/m³");
    insert_derived_unit(&mut m, "mol/m²");
    insert_derived_unit(&mut m, "mol/m");
    insert_derived_unit(&mut m, "m³/mol");
    insert_derived_unit(&mut m, "m³/mol/K");
    insert_derived_unit(&mut m, "g/m³");
    insert_derived_unit(&mut m, "N/m");
    insert_derived_unit(&mut m, "J*s");
    insert_derived_unit(&mut m, "J/mol");
    insert_derived_unit(&mut m, "J/K");
    insert_derived_unit(&mut m, "J/mol/K");
    insert_derived_unit(&mut m, "J/kg");
    insert_derived_unit(&mut m, "J/kg/K");
    insert_derived_unit(&mut m, "Pa*s");
    insert_derived_unit(&mut m, "m/s");
    insert_derived_unit(&mut m, "m²/s");
    insert_derived_unit(&mut m, "W/m/K");
    insert_derived_unit(&mut m, "g/mol");
    insert_derived_unit(&mut m, "m²");
    insert_derived_unit(&mut m, "m³");
    insert_derived_unit(&mut m, "lm/W");
    insert_derived_unit(&mut m, "m³/kg/s²");
    m
});

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_si() {
        assert_eq!(format!("{:.3}", RGAS), "8.314  J/mol/K");
    }

    #[test]
    fn test_fmt_zero() {
        assert_eq!(format!("{}", 0.0 * KELVIN), "0 K");
        assert_eq!(format!("{:.2}", 0.0 * PASCAL), "0.00  Pa");
        assert_eq!((0.0 * KELVIN).to_latex(), "0\\,\\mathrm{K}");
    }
}
