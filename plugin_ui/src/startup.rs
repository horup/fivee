use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use common::{CommonAssets, Round, RoundCommand, Selection, Token, UIDebugFPS, WorldCursor, UI};
pub fn system_ui_startup(mut commands: Commands, common_assets: ResMut<CommonAssets>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 8.0)
            .looking_at(Vec3::new(5.0, 8.0, 0.0), Vec3::Y),
        ..default()
    });

    let font = common_assets.font("default");
    commands
        .spawn(
            TextBundle::from_section(
                "---",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::RED,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        )
        .insert(UIDebugFPS);

    // spawn world cusor
    commands
        .spawn(PbrBundle {
            mesh: common_assets.mesh("selector"),
            material: common_assets.material("white"),
            ..Default::default()
        })
        .insert(WorldCursor::default());
}