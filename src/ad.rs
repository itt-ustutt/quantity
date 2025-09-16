use super::Quantity;
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, OMatrix, OVector, U1};
use num_dual::{
    Dual, Dual2, Dual2Vec, Dual3, DualNum, DualStruct, DualVec, Gradients, HyperDual, HyperDualVec,
    HyperHyperDual, Real,
};
use std::ops::Sub;
use typenum::Diff;

impl<D, F, T: DualStruct<D, F>, U> DualStruct<D, F> for Quantity<T, U> {
    type Real = Quantity<T::Real, U>;
    type Inner = Quantity<T::Inner, U>;

    fn re(&self) -> Self::Real {
        Quantity::new(self.0.re())
    }

    fn from_inner(inner: &Self::Inner) -> Self {
        Quantity::new(T::from_inner(&inner.0))
    }
}

pub fn zeroth_derivative<G, T: DualNum<f64>, UX, UY>(g: G, x: Quantity<T, UX>) -> Quantity<T, UY>
where
    G: Fn(Quantity<Real<T, f64>, UX>) -> Quantity<Real<T, f64>, UY>,
    UY: Sub<UX>,
{
    let r = num_dual::zeroth_derivative(|x| g(Quantity::new(x)).0, x.0);
    Quantity::new(r)
}

pub fn first_derivative<G, T: DualNum<f64>, UX, UY>(
    g: G,
    x: Quantity<T, UX>,
) -> (Quantity<T, UY>, Quantity<T, Diff<UY, UX>>)
where
    G: Fn(Quantity<Dual<T, f64>, UX>) -> Quantity<Dual<T, f64>, UY>,
    UY: Sub<UX>,
{
    let r = num_dual::first_derivative(|x| g(Quantity::new(x)).0, x.0);
    (Quantity::new(r.0), Quantity::new(r.1))
}

#[expect(clippy::type_complexity)]
pub fn gradient<G, T: DualNum<f64>, UX, UY, N: Dim>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (Quantity<T, UY>, Quantity<OVector<T, N>, Diff<UY, UX>>)
where
    DefaultAllocator: Allocator<N>,
    G: Fn(Quantity<OVector<DualVec<T, f64, N>, N>, UX>) -> Quantity<DualVec<T, f64, N>, UY>,
    UY: Sub<UX>,
{
    let r = num_dual::gradient(|x| g(Quantity::new(x)).0, &x.0);
    (Quantity::new(r.0), Quantity::new(r.1))
}

#[expect(clippy::type_complexity)]
pub fn gradient_copy<G, T: DualNum<f64> + Copy, UX, UY, N: Gradients>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (Quantity<T, UY>, Quantity<OVector<T, N>, Diff<UY, UX>>)
where
    DefaultAllocator: Allocator<N>,
    G: Fn(Quantity<OVector<N::Dual<T, f64>, N>, UX>) -> Quantity<N::Dual<T, f64>, UY>,
    UY: Sub<UX>,
{
    let r = N::gradient(|x, _: &()| g(Quantity::new(x)).0, &x.0, &());
    (Quantity::new(r.0), Quantity::new(r.1))
}

#[expect(clippy::type_complexity)]
pub fn jacobian<G, T: DualNum<f64>, UX, UY, M: Dim, N: Dim>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (
    Quantity<OVector<T, M>, UY>,
    Quantity<OMatrix<T, M, N>, Diff<UY, UX>>,
)
where
    DefaultAllocator: Allocator<M> + Allocator<N> + Allocator<M, N> + Allocator<U1, N>,
    G: Fn(
        Quantity<OVector<DualVec<T, f64, N>, N>, UX>,
    ) -> Quantity<OVector<DualVec<T, f64, N>, M>, UY>,
    UY: Sub<UX>,
{
    let r = num_dual::jacobian(|x| g(Quantity::new(x)).0, &x.0);
    (Quantity::new(r.0), Quantity::new(r.1))
}

#[expect(clippy::type_complexity)]
pub fn jacobian_copy<G, T: DualNum<f64> + Copy, UX, UY, N: Gradients>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (
    Quantity<OVector<T, N>, UY>,
    Quantity<OMatrix<T, N, N>, Diff<UY, UX>>,
)
where
    DefaultAllocator: Allocator<N> + Allocator<N> + Allocator<N, N> + Allocator<U1, N>,
    G: Fn(Quantity<OVector<N::Dual<T, f64>, N>, UX>) -> Quantity<OVector<N::Dual<T, f64>, N>, UY>,
    UY: Sub<UX>,
{
    let r = N::jacobian(|x, _: &()| g(Quantity::new(x)).0, &x.0, &());
    (Quantity::new(r.0), Quantity::new(r.1))
}

#[expect(clippy::type_complexity)]
pub fn second_derivative<G, T: DualNum<f64>, UX, UY>(
    g: G,
    x: Quantity<T, UX>,
) -> (
    Quantity<T, UY>,
    Quantity<T, Diff<UY, UX>>,
    Quantity<T, Diff<Diff<UY, UX>, UX>>,
)
where
    G: Fn(Quantity<Dual2<T, f64>, UX>) -> Quantity<Dual2<T, f64>, UY>,
    UY: Sub<UX>,
    Diff<UY, UX>: Sub<UX>,
{
    let r = num_dual::second_derivative(|x| g(Quantity::new(x)).0, x.0);
    (Quantity::new(r.0), Quantity::new(r.1), Quantity::new(r.2))
}

#[expect(clippy::type_complexity)]
pub fn hessian<G, T: DualNum<f64>, UX, UY, N: Dim>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (
    Quantity<T, UY>,
    Quantity<OVector<T, N>, Diff<UY, UX>>,
    Quantity<OMatrix<T, N, N>, Diff<Diff<UY, UX>, UX>>,
)
where
    DefaultAllocator: Allocator<N> + Allocator<U1, N> + Allocator<N, N>,
    G: Fn(Quantity<OVector<Dual2Vec<T, f64, N>, N>, UX>) -> Quantity<Dual2Vec<T, f64, N>, UY>,
    UY: Sub<UX>,
    Diff<UY, UX>: Sub<UX>,
{
    let r = num_dual::hessian(|x| g(Quantity::new(x)).0, &x.0);
    (Quantity::new(r.0), Quantity::new(r.1), Quantity::new(r.2))
}

#[expect(clippy::type_complexity)]
pub fn hessian_copy<G, T: DualNum<f64> + Copy, UX, UY, N: Gradients>(
    g: G,
    x: &Quantity<OVector<T, N>, UX>,
) -> (
    Quantity<T, UY>,
    Quantity<OVector<T, N>, Diff<UY, UX>>,
    Quantity<OMatrix<T, N, N>, Diff<Diff<UY, UX>, UX>>,
)
where
    DefaultAllocator: Allocator<N> + Allocator<N, N>,
    G: Fn(Quantity<OVector<N::Dual2<T, f64>, N>, UX>) -> Quantity<N::Dual2<T, f64>, UY>,
    UY: Sub<UX>,
    Diff<UY, UX>: Sub<UX>,
{
    let r = N::hessian(|x, _: &()| g(Quantity::new(x)).0, &x.0, &());
    (Quantity::new(r.0), Quantity::new(r.1), Quantity::new(r.2))
}

#[expect(clippy::type_complexity)]
pub fn second_partial_derivative<G, T: DualNum<f64>, UX, UY, UZ>(
    g: G,
    (x, y): (Quantity<T, UX>, Quantity<T, UY>),
) -> (
    Quantity<T, UZ>,
    Quantity<T, Diff<UZ, UX>>,
    Quantity<T, Diff<UZ, UY>>,
    Quantity<T, Diff<Diff<UZ, UX>, UY>>,
)
where
    G: Fn(
        (
            Quantity<HyperDual<T, f64>, UX>,
            Quantity<HyperDual<T, f64>, UY>,
        ),
    ) -> Quantity<HyperDual<T, f64>, UZ>,
    UZ: Sub<UX>,
    UZ: Sub<UY>,
    Diff<UZ, UX>: Sub<UY>,
{
    let r = num_dual::second_partial_derivative(
        |(x, y)| g((Quantity::new(x), Quantity::new(y))).0,
        (x.0, y.0),
    );
    (
        Quantity::new(r.0),
        Quantity::new(r.1),
        Quantity::new(r.2),
        Quantity::new(r.3),
    )
}

#[expect(clippy::type_complexity)]
pub fn partial_hessian<G, T: DualNum<f64>, UX, UY, UZ, M: Dim, N: Dim>(
    g: G,
    (x, y): (&Quantity<OVector<T, M>, UX>, &Quantity<OVector<T, N>, UY>),
) -> (
    Quantity<T, UZ>,
    Quantity<OVector<T, M>, Diff<UZ, UX>>,
    Quantity<OVector<T, N>, Diff<UZ, UY>>,
    Quantity<OMatrix<T, M, N>, Diff<Diff<UZ, UX>, UY>>,
)
where
    G: Fn(
        (
            Quantity<OVector<HyperDualVec<T, f64, M, N>, M>, UX>,
            Quantity<OVector<HyperDualVec<T, f64, M, N>, N>, UY>,
        ),
    ) -> Quantity<HyperDualVec<T, f64, M, N>, UZ>,
    DefaultAllocator: Allocator<N> + Allocator<M> + Allocator<M, N> + Allocator<U1, N>,
    UZ: Sub<UX>,
    UZ: Sub<UY>,
    Diff<UZ, UX>: Sub<UY>,
{
    let r = num_dual::partial_hessian(
        |(x, y)| g((Quantity::new(x), Quantity::new(y))).0,
        (&x.0, &y.0),
    );
    (
        Quantity::new(r.0),
        Quantity::new(r.1),
        Quantity::new(r.2),
        Quantity::new(r.3),
    )
}

#[expect(clippy::type_complexity)]
pub fn partial_hessian_copy<G, T: DualNum<f64> + Copy, UX, UY, UZ, N: Gradients>(
    g: G,
    (x, y): (&Quantity<OVector<T, N>, UX>, Quantity<T, UY>),
) -> (
    Quantity<T, UZ>,
    Quantity<OVector<T, N>, Diff<UZ, UX>>,
    Quantity<T, Diff<UZ, UY>>,
    Quantity<OVector<T, N>, Diff<Diff<UZ, UX>, UY>>,
)
where
    G: Fn(
        (
            Quantity<OVector<N::HyperDual<T, f64>, N>, UX>,
            Quantity<N::HyperDual<T, f64>, UY>,
        ),
    ) -> Quantity<N::HyperDual<T, f64>, UZ>,
    DefaultAllocator: Allocator<N>,
    UZ: Sub<UX>,
    UZ: Sub<UY>,
    Diff<UZ, UX>: Sub<UY>,
{
    let r = N::partial_hessian(
        |x, y, _: &()| g((Quantity::new(x), Quantity::new(y))).0,
        &x.0,
        y.0,
        &(),
    );
    (
        Quantity::new(r.0),
        Quantity::new(r.1),
        Quantity::new(r.2),
        Quantity::new(r.3),
    )
}

#[expect(clippy::type_complexity)]
pub fn third_derivative<G, T: DualNum<f64>, UX, UY>(
    g: G,
    x: Quantity<T, UX>,
) -> (
    Quantity<T, UY>,
    Quantity<T, Diff<UY, UX>>,
    Quantity<T, Diff<Diff<UY, UX>, UX>>,
    Quantity<T, Diff<Diff<Diff<UY, UX>, UX>, UX>>,
)
where
    G: Fn(Quantity<Dual3<T, f64>, UX>) -> Quantity<Dual3<T, f64>, UY>,
    UY: Sub<UX>,
    Diff<UY, UX>: Sub<UX>,
    Diff<Diff<UY, UX>, UX>: Sub<UX>,
{
    let r = num_dual::third_derivative(|x| g(Quantity::new(x)).0, x.0);
    (
        Quantity::new(r.0),
        Quantity::new(r.1),
        Quantity::new(r.2),
        Quantity::new(r.3),
    )
}

#[expect(clippy::type_complexity)]
pub fn third_partial_derivative<G, T: DualNum<f64>, UX, UY, UZ, U>(
    g: G,
    (x, y, z): (Quantity<T, UX>, Quantity<T, UY>, Quantity<T, UZ>),
) -> (
    Quantity<T, U>,
    Quantity<T, Diff<U, UX>>,
    Quantity<T, Diff<U, UY>>,
    Quantity<T, Diff<U, UZ>>,
    Quantity<T, Diff<Diff<U, UX>, UY>>,
    Quantity<T, Diff<Diff<U, UX>, UY>>,
    Quantity<T, Diff<Diff<U, UX>, UY>>,
    Quantity<T, Diff<Diff<Diff<U, UX>, UY>, UZ>>,
)
where
    G: Fn(
        (
            Quantity<HyperHyperDual<T, f64>, UX>,
            Quantity<HyperHyperDual<T, f64>, UY>,
            Quantity<HyperHyperDual<T, f64>, UZ>,
        ),
    ) -> Quantity<HyperHyperDual<T, f64>, UY>,
    U: Sub<UX>,
    U: Sub<UY>,
    U: Sub<UZ>,
    Diff<U, UX>: Sub<UY>,
    Diff<U, UY>: Sub<UX>,
    Diff<U, UX>: Sub<UX>,
    Diff<Diff<U, UX>, UY>: Sub<UZ>,
{
    let r = num_dual::third_partial_derivative(
        |(x, y, z)| g((Quantity::new(x), Quantity::new(y), Quantity::new(z))).0,
        (x.0, y.0, z.0),
    );
    (
        Quantity::new(r.0),
        Quantity::new(r.1),
        Quantity::new(r.2),
        Quantity::new(r.3),
        Quantity::new(r.4),
        Quantity::new(r.5),
        Quantity::new(r.6),
        Quantity::new(r.7),
    )
}

#[cfg(test)]
mod test_num_dual {
    use super::*;
    use crate::{Area, Length, Temperature, Volume, METER};
    use approx::assert_relative_eq;
    use nalgebra::{vector, SMatrix, SVector};
    use num_dual::{Dual64, ImplicitDerivative, ImplicitFunction};
    use typenum::{P2, P3};

    struct MyArgs<D> {
        temperature: Temperature<D>,
    }

    impl<D: DualNum<f64> + Copy> DualStruct<D, f64> for MyArgs<D> {
        type Real = MyArgs<f64>;
        type Inner = MyArgs<D::Inner>;

        fn re(&self) -> Self::Real {
            MyArgs {
                temperature: self.temperature.re(),
            }
        }

        fn from_inner(inner: &Self::Inner) -> Self {
            MyArgs {
                temperature: Temperature::from_inner(&inner.temperature),
            }
        }
    }

    struct AreaImplicit;
    impl ImplicitFunction<f64> for AreaImplicit {
        type Parameters<D> = Area<D>;
        type Variable<D> = D;
        fn residual<D: DualNum<f64> + Copy>(x: D, &area: &Area<D>) -> D {
            let l = Length::new(x);
            (area - l * l).convert_into(area)
        }
    }

    #[test]
    fn test_implicit() {
        let a = Area::new(Dual64::from(25.0).derivative());
        let implicit = ImplicitDerivative::new(AreaImplicit, a);
        let x = implicit.implicit_derivative(5.0);
        println!("{x}");
        assert_eq!(x, a.0.sqrt())
    }

    fn volume<D: DualNum<f64> + Copy>(x: Length<D>) -> Volume<D> {
        x * x * x
    }

    fn distance<D: DualNum<f64>, N: Dim>(x: Length<OVector<D, N>>) -> Length<D>
    where
        DefaultAllocator: Allocator<N>,
    {
        x.dot(&x).sqrt()
    }

    fn volume2<D: DualNum<f64> + Copy>((x, h): (Length<D>, Length<D>)) -> Volume<D> {
        x * x * h
    }

    #[test]
    fn test_derivative() {
        let (v, dv) = first_derivative(volume, 5.0 * METER);
        println!("{v}\t{dv:3}");
        assert_eq!(v, 125.0 * METER.powi::<P3>());
        assert_eq!(dv, 75.0 * METER.powi::<P2>());

        let (v, dv, d2v) = second_derivative(volume, 5.0 * METER);
        println!("{v}\t{dv:3}\t\t{d2v}");
        assert_eq!(v, 125.0 * METER.powi::<P3>(),);
        assert_eq!(dv, 75.0 * METER.powi::<P2>(),);
        assert_eq!(d2v, 30.0 * METER);

        let (v, dv_dx, dv_dh, d2v) =
            second_partial_derivative(volume2, (5.0 * METER, 20.0 * METER));
        println!("{v}\t{dv_dx:3}\t{dv_dh:3}\t{d2v}");
        assert_eq!(v, 500.0 * METER.powi::<P3>(),);
        assert_eq!(dv_dx, 200.0 * METER.powi::<P2>(),);
        assert_eq!(dv_dh, 25.0 * METER.powi::<P2>(),);
        assert_eq!(d2v, 10.0 * METER);

        let (v, dv, d2v, d3v) = third_derivative(volume, 5.0 * METER);
        println!("{v}\t{dv:3}\t\t{d2v}\t{d3v}");
        assert_eq!(v, 125.0 * METER.powi::<P3>(),);
        assert_eq!(dv, 75.0 * METER.powi::<P2>(),);
        assert_eq!(d2v, 30.0 * METER);
        assert_eq!(d3v.into_value(), 6.0);
    }

    #[test]
    fn test_gradient_and_hessian() {
        let x = vector![1.0, 5.0, 5.0, 7.0];
        let (d, grad) = gradient(distance, &Length::new(x));
        println!("{d}\t{grad:.1}");
        assert_eq!(d, 10.0 * METER);
        assert_relative_eq!(grad.into_value(), SVector::from([0.1, 0.5, 0.5, 0.7]));

        let x = vector![1.0, 5.0, 5.0, 7.0];
        let (d, grad, hess) = hessian(distance, &Length::new(x));
        println!("{d}\t{grad:.1}\t{hess:.3?}");
        assert_eq!(d, 10.0 * METER);
        assert_relative_eq!(grad.into_value(), SVector::from([0.1, 0.5, 0.5, 0.7]));
        assert_relative_eq!(
            (hess * METER).into_value(),
            SMatrix::from([
                [0.099, -0.005, -0.005, -0.007],
                [-0.005, 0.075, -0.025, -0.035],
                [-0.005, -0.025, 0.075, -0.035],
                [-0.007, -0.035, -0.035, 0.051]
            ])
        );
    }
}
