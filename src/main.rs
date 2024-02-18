//! outputs a series of point represening a
//! gyroid surface in a 1000x1000x1000 space.
//!
//! each point is output as a series of three f32's
//! output is json file with this format
//!
//! [
//!  [
//!    1.0,
//!    2.0,
//!    3.0
//!  ],
//!  [
//!    1.0,
//!    2.0,
//!    3.0
//!  ]
//!]

use std::f64::consts::PI;
use std::f64::consts::TAU;

fn main() {
    // box side length
    let N = 1_00u32;
    let MAX = N as f64;

    let mut points = Vec::<[u32; 3]>::new();

    // JSON format
    // Wrap point in []
    println!("[");
    let delta = 0..1;
    for k in 0..N {
        for j in 0..N {
            for i in 0..N {
                // transform in to space -PI to PI;
                let x = TAU * i as f64 / MAX - PI;
                let y = TAU * j as f64 / MAX - PI;
                let z = TAU * k as f64 / MAX - PI;

                let f = x.sin() * y.cos() + y.sin() * z.cos() + z.sin() * x.cos();
                // if delta.contains(&f) {
                if f.abs() < 0.01_f64 {
                    // current cell is on the surface.
                    points.push([i, j, k]);
                }
            }
        }
    }

    let mut iter = points.iter();
    // first point
    if let Some(first) = iter.next() {
        print!(
            "[{}, {}, {}]",
            first[0] as f64 / 100_f64,
            first[1] as f64 / 100_f64,
            first[2] as f64 / 100_f64
        );

        for p in iter {
            println!(",");
            print!(
                "[ {}, {}, {}]",
                p[0] as f64 / 100_f64,
                p[1] as f64 / 100_f64,
                p[2] as f64 / 100_f64
            );
        }
    }

    // close opening bracket
    println!("]");
}
