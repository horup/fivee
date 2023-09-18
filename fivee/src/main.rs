use std::time::Duration;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode, render::{RenderPlugin, settings::{WgpuSettings}}, asset::ChangeWatcher};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window:Some(Window {
                        present_mode:PresentMode::Immediate,
                        mode:bevy::window::WindowMode::Windowed,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        ..Default::default()
                    },
                }).set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(250)),
                    ..Default::default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(common::CommonPlugin)
        .add_plugins(plugin_assets::PluginAssets)
        .add_plugins(plugin_ui::PluginUI)
        .add_plugins(plugin_game::PluginGame)
        .add_plugins(plugin_ai::PluginAI)
        .run();
}
