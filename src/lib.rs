pub mod feature;
pub mod follow;
pub mod graph;
pub mod read;
pub mod tree;

pub use crate::feature::Element;
pub use crate::graph::{Atom, Bond};

use crate::follow::Writer;
use crate::graph::{walk, Builder};
use crate::read::Error;

pub fn read(smiles: &str) -> Result<Vec<Atom>, Error> {
    let mut graph = Builder::new();
    read::read(smiles, &mut graph)?;
    Ok(graph.build())
}

pub fn write(mol: &[Atom]) -> String {
    let mut writer = Writer::new();
    walk(mol, &mut writer);
    writer.write()
}
