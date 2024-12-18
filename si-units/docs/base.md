# SI Base Units and Associated Constants (AC)

All capitalized entries in the `Unit` and `AC` (associated constants) columns from the table below can be imported:


| Unit     | Symbol       | Quantity                  | AC     | AC Name                                         | AC value                                                       |
| -------- | ------------ | ------------------------- | ------ | ----------------------------------------------- | -------------------------------------------------------------- |
| SECOND   | $\text{s}$   | time                      | DVCS   | Hyperfine transition frequency of Cs            | $\Delta\nu_\text{Cs}=9192631770~\text{Hz}$                     |
| METER    | $\text{m}$   | length                    | CLIGHT | Speed of light                                  | $c=299792458~\frac{\text{m}}{\text{s}}$                        |
| KILOGRAM | $\text{kg}$  | mass                      | PLANCK | Planck constant                                 | $h=6.62607015\times 10^{-34}~\text{J}\cdot\text{s}$            |
| AMPERE   | $\text{A}$   | electric current          | QE     | Elementary charge                               | $e=1.602176634\times 10^{-19}~\text{C}$                        |
| KELVIN   | $\text{K}$   | thermodynamic temperature | KB     | Boltzmann constant                              | $k_\text{B}=1.380649\times 10^{-23}~\frac{\text{J}}{\text{K}}$ |
| MOL      | $\text{mol}$ | amount of substance       | NAV    | Avogadro constant                               | $N_\text{A}=6.02214076\times 10^{23}~\text{mol}^{-1}$          |
| CANDELA  | $\text{cd}$  | luminous intensity        | KCD    | Luminous efficacy of $540~\text{THz}$ radiation | $K_\text{cd}=683~\frac{\text{lm}}{\text{W}}$                   |

## Example

```py linenums="1"
from si_units import SECOND, DVCS
print(SECOND)
print(DVCS)
```
```
1  s
9.19263177 GHz
```