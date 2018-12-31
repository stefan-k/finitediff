// Copyright 2018 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Finite Differentiation
//!
//! TODO: Text.

#![allow(clippy::ptr_arg)]

mod diff;
mod hessian;
mod jacobian;
mod pert;

use crate::diff::*;
use crate::hessian::*;
use crate::jacobian::*;
use crate::pert::*;
#[cfg(feature = "ndarray")]
use ndarray;

/// Ideally, `EPS_F64` should be set to `EPSILON`; however, this caused numerical  problems which
/// where solved by multiplying it with `4.0`. This may require some investigation.
const EPS_F64: f64 = 4.0 * std::f64::EPSILON;

pub trait FiniteDiff
where
    Self: Sized,
{
    type Jacobian;
    type Hessian;
    type OperatorOutput;

    /// Forward difference calculated as
    ///
    /// `df/dx_i (x) \approx (f(x + sqrt(EPS_F64) * e_i) - f(x))/sqrt(EPS_F64)  \forall i`
    ///
    /// where `f` is the cost function and `e_i` is the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `n+1` evaluations of `f`.
    fn forward_diff(&self, f: &Fn(&Self) -> f64) -> Self;

    /// Central difference calculated as
    ///
    /// `df/dx_i (x) \approx (f(x + sqrt(EPS_F64) * e_i) - f(x - sqrt(EPS_F64) * e_i))/(2.0 * sqrt(EPS_F64))  \forall i`
    ///
    /// where `f` is the cost function and `e_i` is the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `2*n` evaluations of `f`.
    fn central_diff(&self, f: &Fn(&Self) -> f64) -> Self;

    /// Calculation of the Jacobian J(x) of a vector function `fs` using forward differences:
    ///
    /// `dfs/dx_i (x) \approx (fs(x + sqrt(EPS_F64) * e_i) - fs(x))/sqrt(EPS_F64)  \forall i`
    ///
    /// where `e_i` is the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `n+1` evaluations of `fs`.
    fn forward_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian;

    /// Calculation of the Jacobian J(x) of a vector function `fs` using central differences:
    ///
    /// `dfs/dx_i (x) \approx (fs(x + sqrt(EPS_F64) * e_i) - fs(x - sqrt(EPS_F64) * e_i))/(2.0 * sqrt(EPS_F64))  \forall i`
    ///
    /// where `e_i` is the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `2*n` evaluations of `fs`.
    fn central_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian;

    /// Calculation of the product of the Jacobian J(x) of a vector function `fs` with a vector `p`
    /// using forward differences:
    ///
    /// `J(x)*p \approx (fs(x + sqrt(EPS_F64) * p) - fs(x))/sqrt(EPS_F64)  \forall i`
    ///
    /// where `e_i` is the `i`th unit vector.
    /// This requires 2 evaluations of `fs`.
    fn forward_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self;

    /// Calculation of the product of the Jacobian J(x) of a vector function `fs` with a vector `p`
    /// using central differences:
    ///
    /// `J(x)*p \approx (fs(x + sqrt(EPS_F64) * p) - fs(x - sqrt(EPS_F64) * p))/(2.0 * sqrt(EPS_F64))  \forall i`
    ///
    /// where `e_i` is the `i`th unit vector.
    /// This requires 2 evaluations of `fs`.
    fn central_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self;

    fn forward_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian;

    fn central_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian;

    /// Calculation of the Hessian using forward differences
    ///
    /// `dg/dx_i (x) \approx (g(x + sqrt(EPS_F64) * e_i) - g(x))/sqrt(EPS_F64)  \forall i`
    ///
    /// where `g` is a function which computes the gradient of some other function f and `e_i` is
    /// the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `n+1` evaluations of `g`.
    fn forward_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Hessian;

    /// Calculation of the Hessian using central differences
    ///
    /// `dg/dx_i (x) \approx (g(x + sqrt(EPS_F64) * e_i) - g(x - sqrt(EPS_F64) * e_i))/(2.0 * sqrt(EPS_F64))  \forall i`
    ///
    /// where `g` is a function which computes the gradient of some other function f and `e_i` is
    /// the `i`th unit vector.
    /// For a parameter vector of length `n`, this requires `2*n` evaluations of `g`.
    fn central_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Hessian;

    /// Calculation of the product of the Hessian H(x) of a function `g` with a vector `p`
    /// using forward differences:
    ///
    /// `H(x)*p \approx (g(x + sqrt(EPS_F64) * p) - g(x))/sqrt(EPS_F64)  \forall i`
    ///
    /// where `g` is a function which computes the gradient of some other function f and `e_i` is
    /// the `i`th unit vector.
    /// This requires 2 evaluations of `g`.
    fn forward_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self;

    /// Calculation of the product of the Hessian H(x) of a function `g` with a vector `p`
    /// using central differences:
    ///
    /// `H(x)*p \approx (g(x + sqrt(EPS_F64) * p) - g(x - sqrt(EPS_F64) * p))/(2.0 * sqrt(EPS_F64))  \forall i`
    ///
    /// where `g` is a function which computes the gradient of some other function f and `e_i` is
    /// the `i`th unit vector.
    /// This requires 2 evaluations of `g`.
    fn central_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self;

    /// Calculation of the Hessian using forward differences without knowledge of the gradient:
    ///
    /// `df/(dx_i dx_j) (x) \approx (f(x + sqrt(EPS_F64) * e_i + sqrt(EPS_F64) * e_j) - f(x + sqrt(EPS_F64) + e_i) - f(x + sqrt(EPS_F64) * e_j) + f(x))/EPS_F64  \forall i`
    ///
    /// where `e_i` and `e_j` are the `i`th and `j`th unit vector, respectively.
    // /// For a parameter vector of length `n`, this requires `n*(n+1)/2` evaluations of `g`.
    fn forward_hessian_nograd(&self, f: &Fn(&Self) -> f64) -> Self::Hessian;

    /// Calculation of a sparse Hessian using forward differences without knowledge of the gradient:
    ///
    /// `df/(dx_i dx_j) (x) \approx (f(x + sqrt(EPS_F64) * e_i + sqrt(EPS_F64) * e_j) - f(x + sqrt(EPS_F64) + e_i) - f(x + sqrt(EPS_F64) * e_j) + f(x))/EPS_F64  \forall i`
    ///
    /// where `e_i` and `e_j` are the `i`th and `j`th unit vector, respectively.
    /// The indices which are to be evaluated need to be provided via `indices`. Note that due to
    /// the symmetry of the Hessian, an index `(a, b)` will also compute the value of the Hessian at
    /// `(b, a)`.
    // /// For a parameter vector of length `n`, this requires `n*(n+1)/2` evaluations of `g`.
    fn forward_hessian_nograd_sparse(
        &self,
        f: &Fn(&Self) -> f64,
        indices: Vec<(usize, usize)>,
    ) -> Self::Hessian;
}

impl FiniteDiff for Vec<f64>
where
    Self: Sized,
{
    type Jacobian = Vec<Vec<f64>>;
    type Hessian = Vec<Vec<f64>>;
    type OperatorOutput = Vec<f64>;

    fn forward_diff(&self, f: &Fn(&Self) -> f64) -> Self {
        forward_diff_vec_f64(self, f)
    }

    fn central_diff(&self, f: &Fn(&Self) -> f64) -> Self {
        central_diff_vec_f64(self, f)
    }

    fn forward_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        forward_jacobian_vec_f64(self, fs)
    }

    fn central_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        central_jacobian_vec_f64(self, fs)
    }

    fn forward_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        forward_jacobian_vec_prod_vec_f64(self, fs, p)
    }

    fn central_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        central_jacobian_vec_prod_vec_f64(self, fs, p)
    }

    fn forward_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian {
        forward_jacobian_pert_vec_f64(self, fs, pert)
    }

    fn central_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian {
        central_jacobian_pert_vec_f64(self, fs, pert)
    }

    fn forward_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Hessian {
        forward_hessian_vec_f64(self, g)
    }

    fn central_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Hessian {
        central_hessian_vec_f64(self, g)
    }

    fn forward_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        forward_hessian_vec_prod_vec_f64(self, g, p)
    }

    fn central_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        central_hessian_vec_prod_vec_f64(self, g, p)
    }

    fn forward_hessian_nograd(&self, f: &Fn(&Self) -> f64) -> Self::Hessian {
        forward_hessian_nograd_vec_f64(self, f)
    }

    fn forward_hessian_nograd_sparse(
        &self,
        f: &Fn(&Self) -> f64,
        indices: Vec<(usize, usize)>,
    ) -> Self::Hessian {
        forward_hessian_nograd_sparse_vec_f64(self, f, indices)
    }
}

#[cfg(feature = "ndarray")]
impl FiniteDiff for ndarray::Array1<f64>
where
    Self: Sized,
{
    type Jacobian = ndarray::Array2<f64>;
    type Hessian = ndarray::Array2<f64>;
    type OperatorOutput = ndarray::Array1<f64>;

    fn forward_diff(&self, f: &Fn(&Self) -> f64) -> Self {
        forward_diff_ndarray_f64(self, f)
    }

    fn central_diff(&self, f: &Fn(&ndarray::Array1<f64>) -> f64) -> Self {
        central_diff_ndarray_f64(self, f)
    }

    fn forward_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        forward_jacobian_ndarray_f64(self, fs)
    }

    fn central_jacobian(&self, fs: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        central_jacobian_ndarray_f64(self, fs)
    }

    fn forward_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        forward_jacobian_vec_prod_ndarray_f64(self, fs, p)
    }

    fn central_jacobian_vec_prod(&self, fs: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        central_jacobian_vec_prod_ndarray_f64(self, fs, p)
    }

    fn forward_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian {
        forward_jacobian_pert_ndarray_f64(self, fs, pert)
    }

    fn central_jacobian_pert(
        &self,
        fs: &Fn(&Self) -> Self::OperatorOutput,
        pert: PerturbationVectors,
    ) -> Self::Jacobian {
        central_jacobian_pert_ndarray_f64(self, fs, pert)
    }

    fn forward_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        forward_hessian_ndarray_f64(self, g)
    }

    fn central_hessian(&self, g: &Fn(&Self) -> Self::OperatorOutput) -> Self::Jacobian {
        central_hessian_ndarray_f64(self, g)
    }

    fn forward_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        forward_hessian_vec_prod_ndarray_f64(self, g, p)
    }

    fn central_hessian_vec_prod(&self, g: &Fn(&Self) -> Self::OperatorOutput, p: &Self) -> Self {
        central_hessian_vec_prod_ndarray_f64(self, g, p)
    }

    fn forward_hessian_nograd(&self, f: &Fn(&Self) -> f64) -> Self::Hessian {
        forward_hessian_nograd_ndarray_f64(self, f)
    }

    fn forward_hessian_nograd_sparse(
        &self,
        f: &Fn(&Self) -> f64,
        indices: Vec<(usize, usize)>,
    ) -> Self::Hessian {
        forward_hessian_nograd_sparse_ndarray_f64(self, f, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "ndarray")]
    use ndarray;

    const COMP_ACC: f64 = 1e-6;

    #[test]
    fn test_forward_diff_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2);
        let p = vec![1.0f64, 1.0f64];
        let grad = p.forward_diff(&f);
        let res = vec![1.0f64, 2.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();

        let p = vec![1.0f64, 2.0f64];
        let grad = p.forward_diff(&f);
        let res = vec![1.0f64, 4.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_diff_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0f64]);

        let grad = p.forward_diff(&f);
        let res = vec![1.0f64, 2.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();

        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0f64]);
        let grad = p.forward_diff(&f);
        let res = vec![1.0f64, 4.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_diff_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2);
        let p = vec![1.0f64, 1.0f64];
        let grad = p.central_diff(&f);
        let res = vec![1.0f64, 2.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();

        let p = vec![1.0f64, 2.0f64];
        let grad = p.central_diff(&f);
        let res = vec![1.0f64, 4.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_diff_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0f64]);

        let grad = p.central_diff(&f);
        let res = vec![1.0f64, 2.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();

        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0f64]);
        let grad = p.central_diff(&f);
        let res = vec![1.0f64, 4.0];

        (0..2)
            .map(|i| assert!((res[i] - grad[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let jacobian = forward_jacobian_vec_f64(&p, &f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let jacobian = p.forward_jacobian(&f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let jacobian = forward_jacobian_ndarray_f64(&p, &f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let jacobian = p.forward_jacobian(&f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let jacobian = central_jacobian_vec_f64(&p, &f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let jacobian = central_jacobian_ndarray_f64(&p, &f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let jacobian = p.central_jacobian(&f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let jacobian = p.central_jacobian(&f);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("{:?}", jacobian);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_vec_prod_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let x = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let p = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let jacobian = forward_jacobian_vec_prod_vec_f64(&x, &f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_vec_prod_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let jacobian = forward_jacobian_vec_prod_ndarray_f64(&x, &f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_vec_prod_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let x = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let p = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let jacobian = x.forward_jacobian_vec_prod(&f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_vec_prod_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let jacobian = x.forward_jacobian_vec_prod(&f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_vec_prod_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let x = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let p = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let jacobian = central_jacobian_vec_prod_vec_f64(&x, &f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_vec_prod_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let jacobian = central_jacobian_vec_prod_ndarray_f64(&x, &f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_vec_prod_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let x = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let p = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let jacobian = x.central_jacobian_vec_prod(&f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_vec_prod_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let jacobian = x.central_jacobian_vec_prod(&f, &p);
        let res = vec![8.0, 22.0, 27.0, 32.0, 37.0, 24.0];
        // println!("{:?}", jacobian);
        // the accuracy for this is pretty bad!!
        (0..6)
            .map(|i| assert!((res[i] - jacobian[i]).abs() < 100.0 * COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_pert_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = forward_jacobian_pert_vec_f64(&p, &f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_pert_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = forward_jacobian_pert_ndarray_f64(&p, &f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_jacobian_pert_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = p.forward_jacobian_pert(&f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_jacobian_pert_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = p.forward_jacobian_pert(&f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_pert_vec_f64() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = central_jacobian_pert_vec_f64(&p, &f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_pert_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = central_jacobian_pert_ndarray_f64(&p, &f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_jacobian_pert_vec_f64_trait() {
        let f = |x: &Vec<f64>| {
            vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ]
        };
        let p = vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = p.central_jacobian_pert(&f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_jacobian_pert_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| {
            ndarray::Array1::from_vec(vec![
                2.0 * (x[1].powi(3) - x[0].powi(2)),
                3.0 * (x[1].powi(3) - x[0].powi(2)) + 2.0 * (x[2].powi(3) - x[1].powi(2)),
                3.0 * (x[2].powi(3) - x[1].powi(2)) + 2.0 * (x[3].powi(3) - x[2].powi(2)),
                3.0 * (x[3].powi(3) - x[2].powi(2)) + 2.0 * (x[4].powi(3) - x[3].powi(2)),
                3.0 * (x[4].powi(3) - x[3].powi(2)) + 2.0 * (x[5].powi(3) - x[4].powi(2)),
                3.0 * (x[5].powi(3) - x[4].powi(2)),
            ])
        };
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let pert = vec![
            PerturbationVector::new()
                .add(0, vec![0, 1])
                .add(3, vec![2, 3, 4]),
            PerturbationVector::new()
                .add(1, vec![0, 1, 2])
                .add(4, vec![3, 4, 5]),
            PerturbationVector::new()
                .add(2, vec![1, 2, 3])
                .add(5, vec![4, 5]),
        ];
        let jacobian = p.central_jacobian_pert(&f, pert);
        let res = vec![
            vec![-4.0, -6.0, 0.0, 0.0, 0.0, 0.0],
            vec![6.0, 5.0, -6.0, 0.0, 0.0, 0.0],
            vec![0.0, 6.0, 5.0, -6.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 5.0, -6.0, 0.0],
            vec![0.0, 0.0, 0.0, 6.0, 5.0, -6.0],
            vec![0.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        ];
        // println!("jacobian:\n{:?}", jacobian);
        // println!("res:\n{:?}", res);
        (0..6)
            .zip(0..6)
            .map(|(i, j)| assert!((res[i][j] - jacobian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = forward_hessian_vec_f64(&p, &|d| d.forward_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = forward_hessian_ndarray_f64(&p, &|d| d.forward_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = p.forward_hessian(&|d| d.forward_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = p.forward_hessian(&|d| d.forward_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_hessian_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = central_hessian_vec_f64(&p, &|d| d.central_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_hessian_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = central_hessian_ndarray_f64(&p, &|d| d.central_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_hessian_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = p.central_hessian(&|d| d.central_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_hessian_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = p.central_hessian(&|d| d.central_diff(&f));
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_vec_prod_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = vec![1.0f64, 1.0, 1.0, 1.0];
        let p = vec![2.0, 3.0, 4.0, 5.0];
        let hessian = forward_hessian_vec_prod_vec_f64(&x, &|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_vec_prod_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![2.0, 3.0, 4.0, 5.0]);
        let hessian = forward_hessian_vec_prod_ndarray_f64(&x, &|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_vec_prod_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = vec![1.0f64, 1.0, 1.0, 1.0];
        let p = vec![2.0, 3.0, 4.0, 5.0];
        let hessian = x.forward_hessian_vec_prod(&|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_vec_prod_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![2.0, 3.0, 4.0, 5.0]);
        let hessian = x.forward_hessian_vec_prod(&|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_hessian_vec_prod_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = vec![1.0f64, 1.0, 1.0, 1.0];
        let p = vec![2.0, 3.0, 4.0, 5.0];
        let hessian = central_hessian_vec_prod_vec_f64(&x, &|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_hessian_vec_prod_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![2.0, 3.0, 4.0, 5.0]);
        let hessian = central_hessian_vec_prod_ndarray_f64(&x, &|d| d.forward_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_central_hessian_vec_prod_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = vec![1.0f64, 1.0, 1.0, 1.0];
        let p = vec![2.0, 3.0, 4.0, 5.0];
        let hessian = x.central_hessian_vec_prod(&|d| d.central_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_central_hessian_vec_prod_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let x = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let p = ndarray::Array1::from_vec(vec![2.0, 3.0, 4.0, 5.0]);
        let hessian = x.central_hessian_vec_prod(&|d| d.central_diff(&f), &p);
        let res = vec![0.0, 6.0, 10.0, 18.0];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .map(|i| assert!((res[i] - hessian[i]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_nograd_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = forward_hessian_nograd_vec_f64(&p, &f);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_nograd_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = forward_hessian_nograd_ndarray_f64(&p, &f);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_nograd_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let hessian = p.forward_hessian_nograd(&f);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_nograd_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let hessian = p.forward_hessian_nograd(&f);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_nograd_sparse_vec_f64() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let indices = vec![(1, 1), (2, 3), (3, 3)];
        let hessian = forward_hessian_nograd_sparse_vec_f64(&p, &f, indices);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_nograd_sparse_ndarray_f64() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let indices = vec![(1, 1), (2, 3), (3, 3)];
        let hessian = forward_hessian_nograd_sparse_ndarray_f64(&p, &f, indices);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }

    #[test]
    fn test_forward_hessian_nograd_sparse_vec_f64_trait() {
        let f = |x: &Vec<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = vec![1.0f64, 1.0, 1.0, 1.0];
        let indices = vec![(1, 1), (2, 3), (3, 3)];
        let hessian = p.forward_hessian_nograd_sparse(&f, indices);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[i][j]).abs() < COMP_ACC))
            .count();
    }

    #[cfg(feature = "ndarray")]
    #[test]
    fn test_forward_hessian_nograd_sparse_ndarray_f64_trait() {
        let f = |x: &ndarray::Array1<f64>| x[0] + x[1].powi(2) + x[2] * x[3].powi(2);
        let p = ndarray::Array1::from_vec(vec![1.0f64, 1.0, 1.0, 1.0]);
        let indices = vec![(1, 1), (2, 3), (3, 3)];
        let hessian = p.forward_hessian_nograd_sparse(&f, indices);
        let res = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 2.0],
            vec![0.0, 0.0, 2.0, 2.0],
        ];
        // println!("hessian:\n{:#?}", hessian);
        // println!("diff:\n{:#?}", diff);
        (0..4)
            .zip(0..4)
            .map(|(i, j)| assert!((res[i][j] - hessian[(i, j)]).abs() < COMP_ACC))
            .count();
    }
}
