use super::*;

impl SIArray1 {
    pub fn gradient(&self, dx: SINumber, left: SINumber, right: SINumber) -> SIArray1 {
        Quantity {
            value: Array::from_shape_fn(self.raw_dim(), |i| {
                let d = if i == 0 {
                    self.value[1] - left.value
                } else if i == self.len() - 1 {
                    right.value - self.value[self.len() - 2]
                } else {
                    self.value[i + 1] - self.value[i - 1]
                };
                d / (2.0 * dx.value)
            }),
            unit: self.unit / dx.unit,
        }
    }
}

impl SIArray2 {
    pub fn gradient(&self, dx: SINumber, left: &SIArray1, right: &SIArray1) -> SIArray2 {
        Quantity {
            value: Array::from_shape_fn(self.raw_dim(), |(c, i)| {
                let d = if i == 0 {
                    self.value[(c, 1)] - left.value[c]
                } else if i == self.len() - 1 {
                    right.value[c] - self.value[(c, self.len() - 2)]
                } else {
                    self.value[(c, i + 1)] - self.value[(c, i - 1)]
                };
                d / (2.0 * dx.value)
            }),
            unit: self.unit / dx.unit,
        }
    }
}

impl<S: Data<Elem = f64>, D: Dimension> Quantity<ArrayBase<S, D>> {
    pub fn integrate(&self, weights: &[SIArray1]) -> SINumber {
        assert_eq!(self.value.ndim(), weights.len());

        let mut value = self.value.to_owned();
        let mut unit = self.unit;
        for (i, w) in weights.iter().enumerate() {
            for mut l in value.lanes_mut(Axis(i)) {
                l.assign(&(&l * &w.value));
            }
            unit *= w.unit;
        }
        Quantity {
            value: value.sum(),
            unit,
        }
    }
}

impl<S: Data<Elem = f64>> Quantity<ArrayBase<S, Ix1>> {
    pub fn integrate_trapezoidal(&self, dx: SINumber) -> SINumber {
        let mut weights = Array::ones(self.len());
        weights[0] = 0.5;
        weights[self.len() - 1] = 0.5;
        self.integrate(&[weights * dx])
    }

    pub fn integrate_trapezoidal_cumulative(&self, dx: SINumber) -> SIArray1 {
        let mut value = Array::zeros(self.len());
        for i in 1..value.len() {
            value[i] = value[i - 1] + (self.value[i - 1] + self.value[i]) * 0.5;
        }
        Quantity {
            value: value * dx.value,
            unit: self.unit * dx.unit,
        }
    }
}
