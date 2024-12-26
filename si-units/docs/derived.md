# Derived Units

| Unit    | Unit symbol | Quantity                         | Definition                             |
| ------- | ----------- | -------------------------------- | -------------------------------------- |
| HERTZ   | $\text{Hz}$ | frequency                        | $\text{s}^{-1}$                        |
| NEWTON  | $\text{N}$  | force; weight                    | $\text{kg}\frac{\text{m}}{\text{s}^2}$ |
| PASCAL  | $\text{Pa}$ | pressure; stress                 | $\frac{\text{N}}{\text{m}^2}$          |
| JOULE   | $\text{J}$  | energy; work; heat               | $\text{N}\text{m}$                     |
| WATT    | $\text{W}$  | power; radiant flux              | $\frac{\text{J}}{\text{s}}$            |
| COULOMB | $\text{C}$  | electric charge                  | $\text{A}\text{s}$                     |
| VOLT    | $\text{V}$  | electrical potential difference  | $\frac{\text{W}}{\text{A}}$            |
| FARAD   | $\text{F}$  | capacitance                      | $\frac{\text{C}}{\text{V}}$            |
| OHM     | $\text{Ω}$  | resistance; impedance; reactance | $\frac{\text{V}}{\text{A}}$            |
| SIEMENS | $\text{S}$  | electrical conductance           | $\text{Ω}^{-1}$                        |
| WEBER   | $\text{Wb}$ | magnetic flux                    | $\text{V}\text{s}$                     |
| TESLA   | $\text{T}$  | magnetic flux density            | $\frac{\text{Wb}}{\text{m}^2}$         |
| HENRY   | $\text{H}$  | inductance                       | $\frac{\text{Wb}}{\text{A}}$           |


## Additional Units

For convenience, a number of commonly used units that are not directly combinations of SI base units is also included.
These constants simplify the specification of properties, that are not given in SI units. However, as the representation
of quantities is unique, they do not appear in formatted outputs.

| Unit     | Unit symbol  | Quantity      | Definition                                    |
| -------- | ------------ | ------------- | --------------------------------------------- |
| ANGSTROM | $\text{Å}$   | length        | $10^{-10}~\text{m}$                           |
| AMU      | $\text{u}$   | mass          | $1.6605390671738466\times 10^{-27}~\text{kg}$ |
| AU       | $\text{au}$  | length        | $149597870700~\text{m}$                       |
| BAR      | $\text{bar}$ | pressure      | $10^5~\text{Pa}$                              |
| CALORIE  | $\text{cal}$ | energy        | $4.184~\text{J}$                              |
| DAY      | $\text{d}$   | time          | $86400~\text{s}$                              |
| DEBYE    | $\text{De}$  | dipole moment | $\sqrt{10^{-19}~\text{JÅ}^3}$                 |
| DEGREES  | $^\circ$     | angle         | $\frac{\pi}{180}~\text{rad}$                  |
| GRAM     | $\text{g}$   | mass          | $10^{-3}~\text{kg}$                           |
| HOUR     | $\text{h}$   | time          | $3600~\text{s}$                               |
| LITER    | $\text{l}$   | volume        | $10^{-3}~\text{m}^3$                          |
| MINUTE   | $\text{min}$ | time          | $60~\text{s}$                                 |
| RADIANS  | $\text{rad}$ | angle         |                                               |

## Additional Constants

| Constant | Name                   | Symbol                    | Value                                                               |
| -------- | ---------------------- | ------------------------- | ------------------------------------------------------------------- |
| G        | Gravitational constant | $G$                       | $6.6743\times 10^{-11}~\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}$ |
| RGAS     | Ideal gas constant     | $R=N_\text{Av}k_\text{B}$ | $8.31446261815324~\frac{\text{J}}{\text{mol}\cdot\text{K}}$         |

### Example

```py linenums="1"
from si_units import RGAS, CALORIE
from si_units import KILO # see prefixes
print(RGAS)
KCAL = KILO * CALORIE
print(KCAL)
```
```
8.31446261815324  J/mol/K
4.184 kJ
```
