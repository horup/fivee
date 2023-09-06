use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct CommonAssets {
    pub materials:HashMap<&'static str, Handle<StandardMaterial>>,
    pub meshes:HashMap<&'static str, Handle<Mesh>>,
    pub images:HashMap<&'static str, Handle<Image>>
}

impl CommonAssets {
    pub fn material(&self, id:&str) -> Handle<StandardMaterial> {
        self.materials.get(id).expect("material not found!").clone()
    }

    pub fn mesh(&self, id:&str) -> Handle<Mesh> {
        self.meshes.get(id).expect("mesh not found!").clone()
    }

    pub fn image(&self, id:&str) -> Handle<Image> {
        self.images.get(id).expect("image not found!").clone()
    }
}
