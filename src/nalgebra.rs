use super::{Quantity, Sum};
use nalgebra::allocator::Allocator;
use nalgebra::constraint::{DimEq, ShapeConstraint};
use nalgebra::{ClosedAddAssign, ClosedMulAssign, DMatrix, DefaultAllocator, Dim, OMatrix, Scalar};
use num_traits::Zero;
use std::marker::PhantomData;
use std::ops::Add;

impl<R: Dim, C: Dim, U, T: Scalar> Quantity<OMatrix<T, R, C>, U>
where
    DefaultAllocator: Allocator<R, C>,
{
    /// Return the total number of elements in the matrix.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return whether the matrix has any elements
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the sum of all elements in the matrix.
    ///
    /// # Example
    /// ```
    /// # use quantity::BAR;
    /// # use nalgebra::dvector;
    /// # use approx::assert_relative_eq;
    /// let x = dvector![1.5, 2.5] * BAR;
    /// assert_relative_eq!(x.sum(), &(4.0 * BAR));
    /// ```
    pub fn sum(&self) -> Quantity<T, U>
    where
        T: Zero + ClosedAddAssign,
    {
        Quantity(self.0.sum(), PhantomData)
    }

    pub fn get(&self, index: usize) -> Quantity<T, U>
    where
        T: Copy,
    {
        Quantity(self.0[index], PhantomData)
    }

    pub fn set(&mut self, index: usize, value: Quantity<T, U>) {
        self.0[index] = value.0;
    }

    pub fn get2(&self, i: usize, j: usize) -> Quantity<T, U>
    where
        T: Copy,
    {
        Quantity(self.0[(i, j)], PhantomData)
    }

    pub fn set2(&mut self, i: usize, j: usize, value: Quantity<T, U>) {
        self.0[(i, j)] = value.0;
    }

    pub fn shape_generic(&self) -> (R, C) {
        self.0.shape_generic()
    }
}

impl<T: Scalar, R: Dim, C: Dim, U> Quantity<OMatrix<T, R, C>, U>
where
    DefaultAllocator: Allocator<R, C>,
{
    pub fn add_scalar(&self, rhs: Quantity<T, U>) -> Self
    where
        T: ClosedAddAssign,
    {
        Self(self.0.add_scalar(rhs.0), PhantomData)
    }

    pub fn component_mul<U2>(
        &self,
        rhs: &Quantity<OMatrix<T, R, C>, U2>,
    ) -> Quantity<OMatrix<T, R, C>, Sum<U, U2>>
    where
        T: ClosedMulAssign,
        U: Add<U2>,
    {
        Quantity(self.0.component_mul(&rhs.0), PhantomData)
    }

    pub fn dot<U2, R2: Dim, C2: Dim>(
        &self,
        rhs: &Quantity<OMatrix<T, R2, C2>, U2>,
    ) -> Quantity<T, Sum<U, U2>>
    where
        DefaultAllocator: Allocator<R2, C2>,
        T: Zero + ClosedAddAssign + ClosedMulAssign,
        U: Add<U2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>,
    {
        Quantity(self.0.dot(&rhs.0), PhantomData)
    }

    pub fn from_fn_generic<F>(nrows: R, ncols: C, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> Quantity<T, U>,
    {
        Self(
            OMatrix::from_fn_generic(nrows, ncols, |i, j| f(i, j).0),
            PhantomData,
        )
    }

    pub fn from_element_generic(nrows: R, ncols: C, elem: Quantity<T, U>) -> Self {
        Self(
            OMatrix::from_element_generic(nrows, ncols, elem.0),
            PhantomData,
        )
    }
}

impl<T: Scalar, U> Quantity<DMatrix<T>, U> {
    pub fn from_fn<F>(nrows: usize, ncols: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> Quantity<T, U>,
    {
        Self(
            DMatrix::from_fn(nrows, ncols, |i, j| f(i, j).0),
            PhantomData,
        )
    }
}
