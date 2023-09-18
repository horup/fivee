use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{AddAsset, App},
    reflect::{TypePath, TypeUuid},
};
use serde::{Deserialize, Serialize};
#[derive(TypeUuid, TypePath, Serialize, Deserialize)]
#[uuid = "f175d5c6-4275-4e40-9105-016d4d0001c1"]
pub struct Statblock {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub speed: u32,
    #[serde(default)]
    pub hit_points:u32
}

#[derive(Default)]
pub struct TomlLoader;

impl AssetLoader for TomlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            match std::str::from_utf8(bytes) {
                Ok(utf8) => {
                    if load_context.path().starts_with("statblocks") {
                        match toml::from_str::<Statblock>(utf8) {
                            Ok(statblock) => {
                                load_context.set_default_asset(LoadedAsset::new(statblock));
                                return Ok(());
                            }
                            Err(err) => {
                                return Err(bevy::asset::Error::msg(err.to_string()));
                            }
                        }
                    } else {
                        return Err(bevy::asset::Error::msg("unknown asset"));
                    }
                   
                },
                Err(err) => return Err(bevy::asset::Error::msg(err.to_string())),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

pub fn build(app: &mut App) {
    app.add_asset::<Statblock>();
    app.init_asset_loader::<TomlLoader>();
}
