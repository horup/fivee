use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct CommonAssets {
    fonts: HashMap<&'static str, Handle<Font>>,
    materials: HashMap<&'static str, Handle<StandardMaterial>>,
    meshes: HashMap<&'static str, Handle<Mesh>>,
    images: HashMap<&'static str, Handle<Image>>,
}

impl CommonAssets {
    pub fn font(&self, id: &str) -> Handle<Font> {
        self.fonts.get(id).expect(&format!("{} font not found!", id)).clone()
    }
    pub fn font_insert(&mut self, id:&'static str, handle:Handle<Font>) {
        self.fonts.insert(id, handle);
    }

    pub fn material(&self, id: &str) -> Handle<StandardMaterial> {
        self.materials.get(id).expect(&format!("{} material not found!", id)).clone()
    }
    pub fn material_insert(&mut self, id:&'static str, handle:Handle<StandardMaterial>) {
        self.materials.insert(id, handle);
    }

    pub fn mesh(&self, id: &str) -> Handle<Mesh> {
        self.meshes.get(id).expect(&format!("{} mesh not found!", id)).clone()
    }
    pub fn mesh_insert(&mut self, id:&'static str, handle:Handle<Mesh>) {
        self.meshes.insert(id, handle);
    }

    pub fn image(&self, id: &str) -> Handle<Image> {
        self.images.get(id).expect(&format!("{} image not found!", id)).clone()
    }
    pub fn image_insert(&mut self, id:&'static str, handle:Handle<Image>) {
        self.images.insert(id, handle);
    }
}
