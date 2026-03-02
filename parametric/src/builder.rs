use std::{collections::HashMap, f32::consts::FRAC_PI_4};

use approx::abs_diff_eq;
use glam::Vec3;
use nrfind::find_root;

use crate::gyroid::Gyroid;

const MAX_ITERATIONS: i32 = 100;

pub(crate) struct GyroidBuilder {
    resolution: u16,
    step: f32,
    epsilon: f32,
}

impl Default for GyroidBuilder {
    fn default() -> Self {
        let resolution = 12u16;
        let step = FRAC_PI_4 / f32::from(resolution);
        Self {
            resolution,
            step,
            epsilon: 1e-4,
        }
    }
}

/// Implicit surface definition F(x, y, z) = 0
fn gyroid(p: Vec3) -> f32 {
    let (sin_x, cos_x) = p.x.sin_cos();
    let (sin_y, cos_y) = p.y.sin_cos();
    let (sin_z, cos_z) = p.z.sin_cos();

    cos_x * sin_y + cos_y * sin_z + cos_z * sin_x
}

impl GyroidBuilder {
    fn is_p_on_gyroid(&self, p: Vec3) -> bool {
        abs_diff_eq!(gyroid(p), 0.0, epsilon = self.epsilon)
    }

    pub(crate) fn build(&self) -> Gyroid<'_> {
        // TODO work in resolution
        let mut lines = HashMap::with_capacity(3);
        // lines.insert("x0", self.x0());
        // lines.insert("y0", self.y0());
        // lines.insert("z0", self.z0());

        let mut patches = HashMap::with_capacity(3);
        patches.insert("patch_x", self.patch_x());
        patches.insert("patch_y", self.patch_y());
        patches.insert("patch_z", self.patch_z());

        Gyroid {
            lines,
            patches,
            resolution: self.resolution,
        }
    }
}

// Surface curves where either X,Y or Z is zero
// allowing a analytical soltion
impl GyroidBuilder {
    // A curve where x=0
    fn x0(&self) -> Vec<Vec3> {
        let mut points = Vec::new();
        // TODO: Complex computaiton using tan(t) and asin()
        // could only compute half this amount
        // and then apply a mirror as tan(t) is an odd function!!!
        let mut t = -FRAC_PI_4 + self.step;
        while t <= FRAC_PI_4 {
            let theta = t.tan();
            let z = -theta.asin();
            let p = Vec3::new(0.0, t, z);
            debug_assert!(self.is_p_on_gyroid(p));
            points.push(p);
            t += self.step;
        }

        points
    }

    // A curve where y=0
    fn y0(&self) -> Vec<Vec3> {
        let mut points = Vec::new();
        let mut t = -FRAC_PI_4 + self.step;
        while t <= FRAC_PI_4 {
            let theta = t.tan();
            let x = -theta.asin();
            let p = Vec3::new(x, 0.0, t);
            debug_assert!(self.is_p_on_gyroid(p));
            points.push(p);
            t += self.step;
        }
        points
    }
    // A curve where z=0
    fn z0(&self) -> Vec<Vec3> {
        let mut points = Vec::new();
        let mut t = -FRAC_PI_4 + self.step;
        while t <= FRAC_PI_4 {
            let theta = t.tan();
            let y = -theta.asin();
            let p = Vec3::new(t, y, 0.0);
            debug_assert!(self.is_p_on_gyroid(p));
            points.push(p);
            t += self.step;
        }
        points
    }
}

impl GyroidBuilder {
    // run t_y, and t_z parameter the full range
    // finding x at every turn;
    fn patch_x(&self) -> Vec<Vec3> {
        let mut points = Vec::with_capacity((self.resolution * self.resolution) as usize);
        // Initial guess for root finder.
        let mut last_x = 0_f32;
        let mut y = -self.step;
        for _u in 0..self.resolution {
            y += self.step;
            let mut z = -self.step;
            for _v in 0..self.resolution {
                z += self.step;
                let gyroid_x = |x: f32| gyroid(Vec3::new(x, y, z));

                // Partial derivative wrt x
                let df_x = |x: f32| {
                    let (sin_x, cos_x) = x.sin_cos();
                    -sin_x * y.sin() + z.cos() * cos_x
                };
                match find_root(&gyroid_x, &df_x, last_x, self.epsilon, MAX_ITERATIONS) {
                    Ok(x) => {
                        let p = Vec3::new(x, y, z);
                        debug_assert!(
                            self.is_p_on_gyroid(p),
                            "Failed: The point p is not on the surface {p} F(x, y, z) = {} ",
                            gyroid(p)
                        );
                        points.push(p);
                        last_x = x;
                    }
                    Err(x) => {
                        panic!(
                            "Failed: Root finder -  Looking for point [x, {y}, {z}] did not find x - last x {x}"
                        );
                    }
                }
            }
        }
        points
    }

    // run t_x, and t_z parameter the full range
    // finding x at every turn;
    fn patch_y(&self) -> Vec<Vec3> {
        let mut points = Vec::with_capacity((self.resolution * self.resolution) as usize);
        // Initial guess for root finder.
        let mut last_y = 0_f32;
        let mut x = -self.step;
        for _u in 0..self.resolution {
            x += self.step;
            let mut z = -self.step;
            for _v in 0..self.resolution {
                z += self.step;

                let gyroid_y = |y: f32| gyroid(Vec3::new(x, y, z));

                // Partial derivative wrt x
                let df_y = |y: f32| x.cos() * y.cos() - y.sin() * z.sin();
                match find_root(&gyroid_y, &df_y, last_y, self.epsilon, MAX_ITERATIONS) {
                    Ok(y) => {
                        let p = Vec3::new(x, y, z);
                        debug_assert!(
                            self.is_p_on_gyroid(p),
                            "Failed: The point p is not on the surface {p} F(x, y, z) = {} ",
                            gyroid(p)
                        );
                        points.push(p);
                        last_y = y;
                    }
                    Err(y) => {
                        panic!(
                            "Failed: Root finder -  Looking for point [{x}, y, {z}] did not find y - last y {y}"
                        );
                    }
                }
            }
        }
        points
    }

    // run t_x, and t_y parameter the full range
    // finding z at every turn;
    fn patch_z(&self) -> Vec<Vec3> {
        let mut points = Vec::with_capacity((self.resolution * self.resolution) as usize);
        // Initial guess for root finder.
        let mut last_z = 0_f32;
        let mut x = -self.step;
        for _u in 0..self.resolution {
            x += self.step;
            let mut y = -self.step;
            for _v in 0..self.resolution {
                y += self.step;
                let gyroid_z = |z: f32| gyroid(Vec3::new(x, y, z));

                // Partial derivative wrt x
                let df_z = |z: f32| y.cos() * z.cos() - z.sin() * x.sin();
                match find_root(&gyroid_z, &df_z, last_z, self.epsilon, MAX_ITERATIONS) {
                    Ok(z) => {
                        let p = Vec3::new(x, y, z);
                        debug_assert!(
                            self.is_p_on_gyroid(p),
                            "Failed: The point p is not on the surface {p} F(x, y, z) = {} ",
                            gyroid(p)
                        );
                        points.push(p);
                        last_z = z;
                    }
                    Err(z) => {
                        panic!(
                            "Failed: Root finder -  Looking for point [{x}, {y} z] did not find z - last z {z}"
                        );
                    }
                }
            }
        }
        points
    }
}
