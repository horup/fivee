use bevy::{reflect::{TypeUuid, TypePath}, asset::{AssetLoader, LoadedAsset}, prelude::{App, AddAsset}};
use serde::{Serialize, Deserialize};
#[derive(TypeUuid, TypePath, Serialize, Deserialize)]
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
        Box::pin(async move {
            match serde_json::from_slice::<Statblock>(bytes) {
                Ok(statblock) => {
                    load_context.set_default_asset(LoadedAsset::new(statblock));
                    return Ok(());
                },
                Err(err) => {
                    return Err(bevy::asset::Error::msg(err.to_string()));
                },
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["statblock"]
    }
}

pub fn build(app:&mut App) {
    app.add_asset::<Statblock>();
    app.init_asset_loader::<StablockAssetLoader>();
}