use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;

fn main() {
    // Generate Neg, Add, Mul, Div, Sub impls for Const<const N: i8>
    // Limiting results of operations to values within [min, max]

    // range of exponents
    // use i32 for results that would overflow i8, we will limit to min/max in loop
    let min: i32 = -20;
    let max: i32 = 20;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_impls.rs");
    let mut out = String::new();

    // go over all exponent combinations
    // for each operation, limit results to min/max
    for a in min..=max {
        // negation
        let neg = -a;
        if neg >= min && neg <= max {
            writeln!(
                &mut out,
                "impl Neg for Const<{a}> {{ type Output = Const<{neg}>; fn neg(self) -> Self::Output {{ Const }} }}"
            ).unwrap();
        }

        for b in min..=max {
            // addition
            let sum = a + b;
            if sum >= min && sum <= max {
                writeln!(
                    &mut out,
                    "impl Add<Const<{b}>> for Const<{a}> {{ type Output = Const<{sum}>; fn add(self, _: Const<{b}>) -> Self::Output {{ Const }} }}"
                ).unwrap();
            }

            // subtraction
            let diff = a - b;
            if diff >= min && diff <= max {
                writeln!(
                    &mut out,
                    "impl Sub<Const<{b}>> for Const<{a}> {{ type Output = Const<{diff}>; fn sub(self, _: Const<{b}>) -> Self::Output {{ Const }} }}"
                ).unwrap();
            }

            // multiplication
            let mul = a * b;
            if mul >= min && mul <= max {
                writeln!(
                    &mut out,
                    "impl Mul<Const<{b}>> for Const<{a}> {{ type Output = Const<{mul}>; fn mul(self, _: Const<{b}>) -> Self::Output {{ Const }} }}"
                ).unwrap();
            }

            // division
            // check: don't divide by 0 and only allow for integer results
            if b != 0 && a % b == 0 {
                let div = a / b;
                if div >= min && div <= max {
                    writeln!(
                            &mut out,
                            "impl Div<Const<{b}>> for Const<{a}> {{ type Output = Const<{div}>; fn div(self, _: Const<{b}>) -> Self::Output {{ Const }} }}"
                        ).unwrap();
                }
            }
        }
    }

    fs::write(&dest_path, out).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
