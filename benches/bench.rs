// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Benches

#![feature(test)]

extern crate finitediff;
extern crate test;

const MASSIVENESS: usize = 512;

fn cost_vec_f64(x: &Vec<f64>) -> f64 {
    x.iter().fold(0.0, |a, acc| a + acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use finitediff::*;
    // use ndarray;
    use test::{black_box, Bencher};

    #[bench]
    fn cost_func_vec_f64(b: &mut Bencher) {
        let x = vec![1.0f64; MASSIVENESS];
        b.iter(|| {
            for _ in 0..MASSIVENESS {
                black_box(cost_vec_f64(&x));
            }
        });
    }

    #[bench]
    fn fwd_diff_vec_f64(b: &mut Bencher) {
        let x = vec![1.0f64; MASSIVENESS];
        b.iter(|| {
            black_box(x.forward_diff(&cost_vec_f64));
        });
    }

}