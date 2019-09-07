use gltf::Gltf;
use std::path::Path;
use crate::{VrmError, Result};

#[derive(Clone, Debug)]
pub struct Vrm {
    model: Gltf,
}

impl Vrm {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Vrm> {
        let model = Gltf::open(path)?;
        Ok(Vrm { model })
    }
}
