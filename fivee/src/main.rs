use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(common::CommonPlugin)
        .add_plugins(plugin_ui::PluginUI)
        .add_plugins(plugin_game::PluginGame)
        .run();
}