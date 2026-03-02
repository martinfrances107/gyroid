use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use glam::Vec3;

// Holds data that can be fed in a GPU for mesh generation.
pub(crate) struct Gyroid<'a> {
    pub(crate) resolution: u16,
    pub(crate) lines: HashMap<&'a str, Vec<Vec3>>,
    pub(crate) patches: HashMap<&'a str, Vec<Vec3>>,
}

impl<'a> Gyroid<'a> {
    pub(crate) fn to_obj_file<W>(&self, buf: &mut BufWriter<W>) -> std::io::Result<()>
    where
        W: Sized + Write,
    {
        // output vertex buffer
        for (name, line) in &self.lines {
            writeln!(buf, "o {name}")?;
            for p in line {
                writeln!(buf, "v {} {} {}", p.x, p.y, p.z)?;
            }
        }

        // let mut offset = 1;
        // for (_obj, lines) in self.lines.iter() {
        //     write!(buf, "l ")?;
        //     for i in 0..lines.len() {
        //         write!(buf, "{} ", i + offset)?;
        //     }
        //     offset += lines.len();
        //     writeln!(buf)?;
        // }

        // patches
        for (name, patch) in &self.patches {
            writeln!(buf, "o {name}")?;
            for p in patch {
                writeln!(buf, "v {} {} {}", p.x, p.y, p.z)?;
            }
        }

        let mut offset: u16 = 1;
        for (_n, p) in &self.patches {
            for u in 0..(self.resolution - 1) {
                for v in 0..(self.resolution - 1) {
                    let a = u * self.resolution + v;
                    let b = u * self.resolution + v + 1;
                    let c = (u + 1) * self.resolution + v;
                    let d = (u + 1) * self.resolution + v + 1;

                    writeln!(buf, "f {} {} {}", a + offset, b + offset, c + offset)?;
                    writeln!(buf, "f {} {} {}", b + offset, d + offset, c + offset)?;
                }
            }
            offset += self.resolution * self.resolution;
        }
        Ok(())
    }
}
