use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window:Some(Window {
                        present_mode:PresentMode::AutoVsync,
                        mode:bevy::window::WindowMode::Windowed,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(common::CommonPlugin)
        .add_plugins(plugin_assets::PluginAssets)
        .add_plugins(plugin_ui::PluginUI)
        .add_plugins(plugin_game::PluginGame)
        .run();
}
