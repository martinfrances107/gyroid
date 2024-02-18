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

fn main() {
    // box side length
    let len = 1_000u32;

    let mut points = Vec::<[u32; 3]>::new();

    // JSON format
    // Wrap point in []
    println!("[");
    let r2 = len * len;
    let shell = r2..r2 + 1;
    for k in 0..len {
        for j in 0..len {
            for i in 0..len {
                let mag = i * i + j * j + k * k;
                if shell.contains(&mag) {
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
