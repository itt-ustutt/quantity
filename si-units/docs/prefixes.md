# Prefixes

All units can be combined with the following prefixes (capitalized):

| Prefix | Prefix symbol | Value      | Prefix | Prefix symbol | Value     |
| ------ | ------------- | ---------- | ------ | ------------- | --------- |
| DECI   | $\text{d}$    | $10^{-1}$  | DECA   | $\text{da}$   | $10^{1}$  |
| CENTI  | $\text{c}$    | $10^{-2}$  | HECTO  | $\text{h}$    | $10^{2}$  |
| MILLI  | $\text{m}$    | $10^{-3}$  | KILO   | $\text{k}$    | $10^{3}$  |
| MICRO  | $\text{µ}$    | $10^{-6}$  | MEGA   | $\text{M}$    | $10^{6}$  |
| NANO   | $\text{n}$    | $10^{-9}$  | GIGA   | $\text{G}$    | $10^{9}$  |
| PICO   | $\text{p}$    | $10^{-12}$ | TERA   | $\text{T}$    | $10^{12}$ |
| FEMTO  | $\text{f}$    | $10^{-15}$ | PETA   | $\text{P}$    | $10^{15}$ |
| ATTO   | $\text{a}$    | $10^{-18}$ | EXA    | $\text{E}$    | $10^{18}$ |
| ZEPTO  | $\text{z}$    | $10^{-21}$ | ZETTA  | $\text{Z}$    | $10^{21}$ |
| YOCTO  | $\text{y}$    | $10^{-24}$ | YOTTA  | $\text{Y}$    | $10^{24}$ |
| RONTO  | $\text{r}$    | $10^{-27}$ | RONNA  | $\text{R}$    | $10^{27}$ |
| QUECTO | $\text{q}$    | $10^{-30}$ | QUETTA | $\text{Q}$    | $10^{30}$ |

## Example

```py linenums="1"
from si_units import MICRO, SECOND, MEGA, PASCAL
dt = 15 * MICRO * SECOND
print(dt)

MPA = MEGA * PASCAL
pressure = 20 * MPA
print(pressure)
```
```
15 µs
20 MPa
```
