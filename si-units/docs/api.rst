API
---

.. currentmodule:: quantity

.. contents:: Table of Contents
    :depth: 3

SI Base Units and Associated Constants
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. csv-table:: Seven SI base units and their associated, exact-valued constants.
    :header: "Unit", "Unit symbol", "Quantity", "Associated constant", "Associated constant name", "Associated constant value"
    :widths: auto

    .. autoclass:: SECOND, :math:`\text{s}`, "time", .. autoclass:: DVCS, "Hyperfine transition frequency of Cs", :math:`\Delta\nu_\text{Cs}=9192631770~\text{Hz}`
    .. autoclass:: METER, :math:`\text{m}`, length, .. autoclass:: CLIGHT, Speed of light, :math:`c=299792458~\frac{\text{m}}{\text{s}}`
    .. autoclass:: KILOGRAM, :math:`\text{kg}`, mass, .. autoclass:: PLANCK, Planck constant, :math:`h=6.62607015\times 10^{-34}~\text{J}\cdot\text{s}`
    .. autoclass:: AMPERE, :math:`\text{A}`, electric current, .. autoclass:: QE, Elementary charge, :math:`e=1.602176634\times 10^{-19}~\text{C}`
    .. autoclass:: KELVIN, :math:`\text{K}`, thermodynamic temperature, .. autoclass:: KB, Boltzmann constant, :math:`k_\text{B}=1.380649\times 10^{-23}~\frac{\text{J}}{\text{K}}`
    .. autoclass:: MOL, :math:`\text{mol}`, amount of substance, .. autoclass:: NAV, Avogadro constant, :math:`N_\text{A}=6.02214076\times 10^{23}~\text{mol}^{-1}`
    .. autoclass:: CANDELA, :math:`\text{cd}`, luminous intensity, .. autoclass:: KCD, Luminous efficacy of :math:`540~\text{THz}` radiation, :math:`K_\text{cd}=683~\frac{\text{lm}}{\text{W}}`

Derived Units
~~~~~~~~~~~~~

.. csv-table::
    :header: "Unit", "Unit symbol", "Quantity", "Definition"
    :widths: auto

    .. autoclass:: HERTZ, :math:`\text{Hz}`, frequency, :math:`\text{s}^{-1}`
    .. autoclass:: NEWTON, :math:`\text{N}`, force; weight, :math:`\text{kg}\frac{\text{m}}{\text{s}^2}`
    .. autoclass:: PASCAL, :math:`\text{Pa}`, pressure; stress, :math:`\frac{\text{N}}{\text{m}^2}`
    .. autoclass:: JOULE, :math:`\text{J}`, energy; work; heat, :math:`\text{N}\text{m}`
    .. autoclass:: WATT, :math:`\text{W}`, power; radiant flux, :math:`\frac{\text{J}}{\text{s}}`
    .. autoclass:: COULOMB, :math:`\text{C}`, electric charge, :math:`\text{A}\text{s}`
    .. autoclass:: VOLT, :math:`\text{V}`, electrical potential difference, :math:`\frac{\text{W}}{\text{A}}`
    .. autoclass:: FARAD, :math:`\text{F}`, capacitance, :math:`\frac{\text{C}}{\text{V}}`
    .. autoclass:: OHM, :math:`\text{Ω}`, resistance; impedance; reactance, :math:`\frac{\text{V}}{\text{A}}`
    .. autoclass:: SIEMENS, :math:`\text{S}`, electrical conductance, :math:`\text{Ω}^{-1}`
    .. autoclass:: WEBER, :math:`\text{Wb}`, magnetic flux, :math:`\text{V}\text{s}`
    .. autoclass:: TESLA, :math:`\text{T}`, magnetic flux density, :math:`\frac{\text{Wb}}{\text{m}^2}`
    .. autoclass:: HENRY, :math:`\text{H}`, inductance, :math:`\frac{\text{Wb}}{\text{A}}`

Additional Units
~~~~~~~~~~~~~~~~

For convenience, a number of commonly used units that are not directly combinations of SI base units is also included.
These constants simplify the specification of properties, that are not given in SI units. However, as the representation
of quantities is unique, they do not appear in formatted outputs.

.. csv-table::
    :header: "Unit", "Unit symbol", "Quantity", "Definition"
    :widths: auto

    .. autoclass:: ANGSTROM, :math:`\text{Å}`, length, :math:`10^{-10}~\text{m}`
    .. autoclass:: AMU, :math:`\text{u}`, mass, :math:`1.6605390671738466\times 10^{-27}~\text{kg}`
    .. autoclass:: AU, :math:`\text{au}`, length, :math:`149597870700~\text{m}`
    .. autoclass:: BAR, :math:`\text{bar}`, pressure, :math:`10^5~\text{Pa}`
    .. autoclass:: CALORIE, :math:`\text{cal}`, energy, :math:`4.184~\text{J}`
    .. autoclass:: DAY, :math:`\text{d}`, time, :math:`86400~\text{s}`
    .. autoclass:: DEGREES, :math:`^\circ`, angle, :math:`\frac{\pi}{180}~\text{rad}`
    .. autoclass:: GRAM, :math:`\text{g}`, mass, :math:`10^{-3}~\text{kg}`
    .. autoclass:: HOUR, :math:`\text{h}`, time, :math:`3600~\text{s}`
    .. autoclass:: LITER, :math:`\text{l}`, volume, :math:`10^{-3}~\text{m}^3`
    .. autoclass:: MINUTE, :math:`\text{min}`, time, :math:`60~\text{s}`
    .. autoclass:: RADIANS, :math:`\text{rad}`, angle,

Additional Constants
~~~~~~~~~~~~~~~~~~~~

.. csv-table::
    :header: "Constant", "Name", "Symbol", "Value"
    :widths: auto

    .. autoclass:: G, Gravitational constant, :math:`G`, :math:`6.6743\times 10^{-11}~\frac{\text{m}^3}{\text{kg}\cdot\text{s}^2}`
    .. autoclass:: RGAS, Ideal gas constant, :math:`R=N_\text{Av}k_\text{B}`, :math:`8.31446261815324~\frac{\text{J}}{\text{mol}\cdot\text{K}}`

Prefixes
~~~~~~~~

All units can be combined with the following prefixes:

.. csv-table::
    :header: "Prefix", "Prefix symbol", "Value", "Prefix", "Prefix symbol", "Value"
    :widths: auto

    .. autoclass:: DECI, :math:`\text{d}`, :math:`10^{-1}`, .. autoclass:: DECA, :math:`\text{da}`, :math:`10^{1}`
    .. autoclass:: CENTI, :math:`\text{c}`, :math:`10^{-2}`, .. autoclass:: HECTO, :math:`\text{h}`, :math:`10^{2}`
    .. autoclass:: MILLI, :math:`\text{m}`, :math:`10^{-3}`, .. autoclass:: KILO, :math:`\text{k}`, :math:`10^{3}`
    .. autoclass:: MICRO, :math:`\text{µ}`, :math:`10^{-6}`, .. autoclass:: MEGA, :math:`\text{M}`, :math:`10^{6}`
    .. autoclass:: NANO, :math:`\text{n}`, :math:`10^{-9}`, .. autoclass:: GIGA, :math:`\text{G}`, :math:`10^{9}`
    .. autoclass:: PICO, :math:`\text{p}`, :math:`10^{-12}`, .. autoclass:: TERA, :math:`\text{T}`, :math:`10^{12}`
    .. autoclass:: FEMTO, :math:`\text{f}`, :math:`10^{-15}`, .. autoclass:: PETA, :math:`\text{P}`, :math:`10^{15}`
    .. autoclass:: ATTO, :math:`\text{a}`, :math:`10^{-18}`, .. autoclass:: EXA, :math:`\text{E}`, :math:`10^{18}`
    .. autoclass:: ZEPTO, :math:`\text{z}`, :math:`10^{-21}`, .. autoclass:: ZETTA, :math:`\text{Z}`, :math:`10^{21}`
    .. autoclass:: YOCTO, :math:`\text{y}`, :math:`10^{-24}`, .. autoclass:: YOTTA, :math:`\text{Y}`, :math:`10^{24}`

Datatypes
~~~~~~~~~

.. autosummary::
    :toctree: generated/

    SINumber
    SIArray1
    SIArray2
    SIArray3
    SIArray4
