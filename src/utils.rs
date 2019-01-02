// Copyright 2018 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[inline(always)]
pub fn mod_and_calc_vec_f64<T>(x: &mut Vec<f64>, f: &Fn(&Vec<f64>) -> T, idx: usize, y: f64) -> T {
    let xtmp = x[idx];
    x[idx] = xtmp + y;
    let fx1 = (f)(&x);
    x[idx] = xtmp;
    fx1
}

#[cfg(feature = "ndarray")]
#[inline(always)]
pub fn mod_and_calc_ndarray_f64<T>(
    x: &mut ndarray::Array1<f64>,
    f: &Fn(&ndarray::Array1<f64>) -> T,
    idx: usize,
    y: f64,
) -> T {
    let xtmp = x[idx];
    x[idx] = xtmp + y;
    let fx1 = (f)(&x);
    x[idx] = xtmp;
    fx1
}

#[inline(always)]
pub fn restore_symmetry_vec_f64(mut mat: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for i in 0..mat.len() {
        for j in (i + 1)..mat[i].len() {
            let t = (mat[i][j] + mat[j][i]) / 2.0;
            mat[i][j] = t;
            mat[j][i] = t;
        }
    }
    mat
}

#[cfg(feature = "ndarray")]
#[inline(always)]
pub fn restore_symmetry_ndarray_f64(mut mat: ndarray::Array2<f64>) -> ndarray::Array2<f64> {
    let (nx, ny) = mat.dim();
    for i in 0..nx {
        for j in (i + 1)..ny {
            let t = (mat[(i, j)] + mat[(j, i)]) / 2.0;
            mat[(i, j)] = t;
            mat[(j, i)] = t;
        }
    }
    mat
}
