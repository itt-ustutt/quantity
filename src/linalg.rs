use super::*;

impl<U: Unit> QuantityArray1<U> {
    pub fn gradient(
        &self,
        dx: QuantityScalar<U>,
        left: QuantityScalar<U>,
        right: QuantityScalar<U>,
    ) -> QuantityArray1<U> {
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

impl<U: Unit> QuantityArray2<U> {
    pub fn gradient(
        &self,
        dx: QuantityScalar<U>,
        left: &QuantityArray1<U>,
        right: &QuantityArray1<U>,
    ) -> QuantityArray2<U> {
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

impl<S: Data<Elem = f64>, D: Dimension, U: Unit> Quantity<ArrayBase<S, D>, U> {
    pub fn integrate(&self, weights: &[QuantityArray1<U>]) -> QuantityScalar<U> {
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

impl<S: Data<Elem = f64>, U: Unit> Quantity<ArrayBase<S, Ix1>, U> {
    pub fn integrate_trapezoidal(&self, dx: QuantityScalar<U>) -> QuantityScalar<U> {
        let mut weights = Array::ones(self.len());
        weights[0] = 0.5;
        weights[self.len() - 1] = 0.5;
        self.integrate(&[weights * dx])
    }

    pub fn integrate_trapezoidal_cumulative(&self, dx: QuantityScalar<U>) -> QuantityArray1<U> {
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
