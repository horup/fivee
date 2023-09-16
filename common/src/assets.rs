use bevy::{reflect::{TypeUuid, TypePath}, asset::AssetLoader, prelude::{App, AddAsset}};

#[derive(TypeUuid, TypePath)]
#[uuid = "f175d5c6-4275-4e40-9105-016d4d0001c1"]
pub struct Statblock {
    pub movement_ft: Option<f32>,
}

#[derive(Default)]
pub struct StablockAssetLoader;

impl AssetLoader for StablockAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["statblock"]
    }
}

pub fn build(app:&mut App) {
    app.init_asset_loader::<StablockAssetLoader>();
}