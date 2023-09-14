use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use common::{CommonAssets, Round, RoundCommand, Selection, Token, Grid};

mod components;
pub use components::*;
mod systems;
pub use systems::*;
mod events;
pub use events::*;

pub struct PluginUI;
impl Plugin for PluginUI {
    fn build(&self, app: &mut App) {
        app.add_event::<GridCursorEvent>();
        app.insert_resource(UI::default());
        add_systems(app);
    }
}
