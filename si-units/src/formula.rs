//! Formula API: parse strings like `"kWh"`, `"J/mol/K"`, `"kg/m^3"` into a
//! `(scale, SIUnit)` pair.
//!
//! ## Matching rules
//!
//! Each atom is matched greedily. The scanner picks the interpretation that
//! consumes the most characters, with ties going to the **bare unit** so
//! canonical symbols (`kg`, `cd`, `mol`, `min`, `Pa`) keep their primary
//! meaning. Otherwise the symbol is split into a prefix + unit (`mm`, `kJ`,
//! `kWh`, `\u{00B5}m`).
//!
//! Every unit has at least one ASCII-only spelling. Non-ASCII forms are
//! retained for ergonomics.

use super::*;
use pyo3::exceptions::PyValueError;

#[rustfmt::skip]
const PREFIXES: &[(&str, f64)] = &[
    // 2 bytes (\u{00B5} = micro sign U+00B5, 2 bytes UTF-8).
    ("da", DECA), ("\u{00B5}", MICRO),
    // 1 byte.
    ("Q", QUETTA),
    ("R", RONNA),
    ("Y", YOTTA),
    ("Z", ZETTA),
    ("E", EXA),
    ("P", PETA),
    ("T", TERA),
    ("G", GIGA),
    ("M", MEGA),
    ("k", KILO),
    ("h", HECTO),
    ("d", DECI),
    ("c", CENTI),
    ("m", MILLI),
    ("u", MICRO),
    ("n", NANO),
    ("p", PICO),
    ("f", FEMTO),
    ("a", ATTO),
    ("z", ZEPTO),
    ("y", YOCTO),
    ("r", RONTO),
    ("q", QUECTO),
];

#[rustfmt::skip]
const UNITS: &[(&str, f64, SIUnit)] = &[
    // 3 bytes.
    ("mol", 1.0, _MOL),
    ("min", 60.0, _SECOND),
    ("cal", 4.184, _JOULE),
    ("bar", 1e5, _PASCAL),
    ("Ang", 1e-10, _METER),
    ("ang", 1e-10, _METER),
    ("Ohm", 1.0, _OHM),
    ("ohm", 1.0, _OHM),
    // 2 bytes (\u{00C5} = angstrom sign U+00C5, \u{03A9} = ohm sign U+03A9).
    ("Pa", 1.0, _PASCAL),
    ("Hz", 1.0, _HERTZ),
    ("Wb", 1.0, _WEBER),
    ("cd", 1.0, _CANDELA),
    ("kg", 1.0, _KILOGRAM),
    ("\u{00C5}", 1e-10, _METER),
    ("\u{03A9}", 1.0, _OHM),
    // 1 byte.
    ("m", 1.0, _METER),
    ("g", 1e-3, _KILOGRAM),
    ("s", 1.0, _SECOND),
    ("A", 1.0, _AMPERE),
    ("K", 1.0, _KELVIN),
    ("N", 1.0, _NEWTON),
    ("J", 1.0, _JOULE),
    ("W", 1.0, _WATT),
    ("C", 1.0, _AMPERE_SECOND),
    ("V", 1.0, _VOLT),
    ("F", 1.0, _FARAD),
    ("S", 1.0, _SIEMENS),
    ("T", 1.0, _TESLA),
    ("H", 1.0, _HENRY),
    ("L", 1e-3, _CUBIC_METER),
    ("l", 1e-3, _CUBIC_METER),
    ("h", 3600.0, _SECOND),
    ("d", 86400.0, _SECOND),
];

// Power-shortcut characters.
const SUPER_2: &str = "\u{00B2}";
const SUPER_3: &str = "\u{00B3}";

// `SIUnit` stores each component as `i8`, so per-atom exponents must fit there
// to avoid silent truncation in `SIUnit::powi`.
const MIN_EXPONENT: i32 = i8::MIN as i32;
const MAX_EXPONENT: i32 = i8::MAX as i32;

/// One matched atom together with the unconsumed input remainder.
struct Atom<'a> {
    factor: f64,
    unit: SIUnit,
    rest: &'a str,
}

// Lookup primitives.
fn match_unit(s: &str) -> Option<Atom<'_>> {
    UNITS.iter().find_map(|&(sym, factor, unit)| {
        s.strip_prefix(sym).map(|rest| Atom { factor, unit, rest })
    })
}

fn match_prefix(s: &str) -> Option<(f64, &str)> {
    PREFIXES
        .iter()
        .find_map(|&(sym, factor)| s.strip_prefix(sym).map(|rest| (factor, rest)))
}

/// Match one atom: prefer the interpretation that consumes more characters,
/// breaking ties in favor of the bare unit so canonical symbols (`kg`, `cd`,
/// `mol`, `min`, `Pa`) keep their primary meaning.
fn match_atom(s: &str) -> Option<Atom<'_>> {
    let bare = match_unit(s);
    let prefixed = match_prefix(s).and_then(|(pf, after)| {
        match_unit(after).map(|atom| Atom {
            factor: pf * atom.factor,
            ..atom
        })
    });
    match (bare, prefixed) {
        (Some(b), Some(p)) if p.rest.len() < b.rest.len() => Some(p),
        (b @ Some(_), _) => b,
        (None, p) => p,
    }
}

// Parsers.
fn read_int(s: &str) -> Option<(i32, &str)> {
    let s = s.trim_start();
    let (sign, rest) = match s.as_bytes().first()? {
        b'-' => (-1, &s[1..]),
        b'+' => (1, &s[1..]),
        _ => (1, s),
    };
    let digits = rest.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    let n: i32 = rest[..digits].parse().ok()?;
    Some((sign * n, &rest[digits..]))
}

fn parse_power(s: &mut &str) -> Result<i32, String> {
    if let Some(rest) = s.strip_prefix("**").or_else(|| s.strip_prefix('^')) {
        let (n, rest) = read_int(rest).ok_or_else(|| format!("expected integer power at '{s}'"))?;
        *s = rest.trim_start();
        Ok(n)
    } else if let Some(rest) = s.strip_prefix(SUPER_2) {
        *s = rest.trim_start();
        Ok(2)
    } else if let Some(rest) = s.strip_prefix(SUPER_3) {
        *s = rest.trim_start();
        Ok(3)
    } else {
        Ok(1)
    }
}

/// Returns `1` for explicit or implicit multiplication and `-1` after `/`.
fn parse_separator(s: &mut &str) -> i32 {
    if let Some(rest) = s.strip_prefix('*') {
        *s = rest;
        1
    } else if let Some(rest) = s.strip_prefix('/') {
        *s = rest;
        -1
    } else {
        1
    }
}

fn parse_unit_expr(input: &str) -> Result<(f64, SIUnit), String> {
    let mut s = input.trim();
    if s.is_empty() {
        return Err("empty unit expression".into());
    }
    let mut factor = 1.0_f64;
    let mut unit = SIUnit::DIMENSIONLESS;
    let mut sign: i32 = 1;

    while !s.is_empty() {
        s = s.trim_start();
        let atom = match_atom(s).ok_or_else(|| format!("unknown unit at '{s}'"))?;
        s = atom.rest.trim_start();

        let exponent = sign * parse_power(&mut s)?;
        if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
            return Err(format!(
                "exponent {exponent} outside [{MIN_EXPONENT}, {MAX_EXPONENT}]"
            ));
        }
        factor *= atom.factor.powi(exponent);
        unit *= atom.unit.powi(exponent);

        sign = parse_separator(&mut s);
    }
    Ok((factor, unit))
}

/// PyO3 entry point: build a quantity from a numeric value and a unit
/// expression. See module docs for grammar and recognised symbols.
#[pyfunction]
pub(crate) fn declare_unit<'py>(
    py: Python<'py>,
    value: f64,
    expr: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let (factor, unit) = parse_unit_expr(expr).map_err(PyValueError::new_err)?;
    let scaled = PyFloat::new(py, value * factor).into_any();
    PySIObject::new_unit_checked(scaled, unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> (f64, SIUnit) {
        parse_unit_expr(s).unwrap()
    }

    #[test]
    fn test_lookup_tables_entries_sorted_by_byte_length_desc() {
        for w in PREFIXES.windows(2) {
            assert!(
                w[0].0.len() >= w[1].0.len(),
                "PREFIXES misordered: {:?} before {:?}",
                w[0].0,
                w[1].0,
            );
        }
        for w in UNITS.windows(2) {
            assert!(
                w[0].0.len() >= w[1].0.len(),
                "UNITS misordered: {:?} before {:?}",
                w[0].0,
                w[1].0,
            );
        }
    }

    #[test]
    fn test_lookup_tables_every_non_ascii_entry_has_ascii_alias() {
        for &(sym, factor) in PREFIXES {
            if !sym.is_ascii() {
                assert!(
                    PREFIXES.iter().any(|&(s, f)| s.is_ascii() && f == factor),
                    "non-ASCII prefix {sym:?} has no ASCII alias",
                );
            }
        }
        for &(sym, factor, unit) in UNITS {
            if !sym.is_ascii() {
                assert!(
                    UNITS
                        .iter()
                        .any(|&(s, f, u)| s.is_ascii() && f == factor && u == unit),
                    "non-ASCII unit {sym:?} has no ASCII alias",
                );
            }
        }
    }

    #[test]
    fn test_parse_bare_symbol_returns_canonical_si_unit() {
        assert_eq!(parse("m").1, _METER);
        assert_eq!(parse("kg"), (1.0, _KILOGRAM));
        assert_eq!(parse("Pa").1, _PASCAL);
        assert_eq!(parse("mol").1, _MOL);
        assert_eq!(parse("cd").1, _CANDELA);
        assert_eq!(parse("min"), (60.0, _SECOND));
        assert_eq!(parse("h"), (3600.0, _SECOND));
        assert_eq!(parse("d"), (86400.0, _SECOND));
    }

    #[test]
    fn test_parse_prefixed_symbol_returns_scaled_si_unit() {
        assert_eq!(parse("mm"), (MILLI, _METER));
        assert_eq!(parse("km"), (KILO, _METER));
        assert_eq!(parse("kJ"), (KILO, _JOULE));
        assert_eq!(parse("um"), (MICRO, _METER));
        assert_eq!(parse("\u{00B5}m"), (MICRO, _METER));
    }

    #[test]
    fn test_parse_ambiguous_symbol_resolves_to_bare_unit_on_tie() {
        // `cd` is candela rather than centi*day even though both
        // interpretations consume two characters.
        assert_eq!(parse("cd").1, _CANDELA);
        // `kg` stays kilogram instead of kilo*gram.
        assert_eq!(parse("kg"), (1.0, _KILOGRAM));
    }

    #[test]
    fn test_parse_ascii_alias_matches_unicode_form() {
        assert_eq!(parse("Ang"), parse("\u{00C5}"));
        assert_eq!(parse("ang"), parse("\u{00C5}"));
        assert_eq!(parse("Ohm"), parse("\u{03A9}"));
        assert_eq!(parse("ohm"), parse("\u{03A9}"));
    }

    #[test]
    fn test_parse_power_operator_returns_unit_raised_to_power() {
        assert_eq!(parse("m**2"), (1.0, _METER.powi(2)));
        assert_eq!(parse("m^2"), (1.0, _METER.powi(2)));
        assert_eq!(parse("m\u{00B2}"), (1.0, _METER.powi(2)));
        assert_eq!(parse("m\u{00B3}"), (1.0, _METER.powi(3)));
        assert_eq!(parse("m**-1"), (1.0, _METER.powi(-1)));
    }

    #[test]
    fn test_parse_composite_expression_returns_combined_unit() {
        assert_eq!(parse("J/mol/K").1, _JOULE_PER_MOL_AND_KELVIN);
        let (f, u) = parse("kWh");
        assert_eq!(u, _JOULE);
        assert!((f - 3.6e6).abs() < 1.0);
        let (f, u) = parse("kg m**2 / s**2");
        assert_eq!(u, _JOULE);
        assert!((f - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_parse_invalid_expression_returns_error() {
        assert!(parse_unit_expr("").is_err());
        assert!(parse_unit_expr("xyz").is_err());
        assert!(parse_unit_expr("m**").is_err());
    }

    #[test]
    fn test_parse_exponent_out_of_range_returns_error() {
        assert!(parse_unit_expr("m**200").is_err());
        assert!(parse_unit_expr("m**-200").is_err());
    }
}
