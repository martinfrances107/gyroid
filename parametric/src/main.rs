pub(crate) mod builder;
pub(crate) mod gyroid;

use std::io;

use builder::GyroidBuilder;
fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let gb = GyroidBuilder::default();
    let gyroid = gb.build();

    gyroid.to_obj_file(&mut handle)?;
    Ok(())
}
