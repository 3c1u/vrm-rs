pub mod json;
pub mod vrm;

use gltf;

#[derive(Debug)]
pub enum VrmError {
    UnknownError,
    GltfError(gltf::Error),
}

impl From<gltf::Error> for VrmError {
    fn from(e: gltf::Error) -> VrmError {
        Self::GltfError(e)
    }
}

pub type Result<T> = std::result::Result<T, VrmError>;

fn main() {
    // load .vrm file
    let vrm_file = "Victoria_Rubin.vrm";

    let model = gltf::Gltf::open(vrm_file).unwrap();
    println!("{:?}", model);
}
