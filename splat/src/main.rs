use core::f64::consts::{PI, TAU};

fn main() {
    // box side length
    const N: u32 = 1_500u32;
    const MAX: f64 = N as f64;

    let mut points = Vec::<[u32; 3]>::new();

    // JSON format
    // Wrap point in []
    println!("[");

    let delta = 0..1;
    for k in 0..N {
        let z = TAU * k as f64 / MAX - PI;
        for j in 0..N {
            let y = TAU * j as f64 / MAX - PI;
            for i in 0..N {
                // transform in to space -PI to PI;
                let x = TAU * i as f64 / MAX - PI;

                let f = x.sin() * y.cos() + y.sin() * z.cos() + z.sin() * x.cos();
                if f.abs() < 0.0001_f64 {
                    // Current cell is on the surface.
                    points.push([i, j, k]);
                }
            }
        }
    }

    //output header


    let mut iter = points.iter();
    // first point
    if let Some(first) = iter.next() {
        print!("[{}, {}, {}]", first[0], first[1], first[2]);

        for p in iter {
            println!(",");
            print!("[ {}, {}, {}]", p[0], p[1], p[2]);
        }
    }

    // close opening bracket
    println!("]");
}
