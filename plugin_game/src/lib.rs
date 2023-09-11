use bevy::prelude::*;
use common::{CommonAssets, Token, Round, RoundCommand, Grid};
mod startup;
pub use startup::*;


fn spawned_token(mut commands:Commands, q:Query<(Entity, &Token), Added<Token>>, sa:Res<CommonAssets>, mut materials:ResMut<Assets<StandardMaterial>>) {
    for (e, token) in q.iter() {
        commands.entity(e).insert(PbrBundle {
            transform:Transform::from_translation(Token::pos(token.grid_pos)),
            mesh:sa.mesh("token"),
            material:materials.add(StandardMaterial {
                base_color:token.color,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

fn update_round_command(command:&mut RoundCommand, round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
    match command.variant {
        common::Variant::Nop => {},
        common::Variant::MoveTo { who, to } => {
            let a = command.alpha();
            if let Ok(token) = tokens.get(who) {
                if let Ok(mut transform) = transforms.get_mut(who) {
                    let s = Token::pos(token.grid_pos);
                    let e = Token::pos(to);
                    let v = e - s;
                    let v = v * common::math::smootherstep(0.0, 1.0, a); 
                    let mut z = 0.0;
                    if a <= 0.5 {
                        z = a * 2.0;
                    } else {
                        z = 1.0 - (a - 0.5) * 2.0;
                    }
                    z*=0.2;
                    transform.translation = s + v + Vec3::new(0.0, 0.0, z);
                }
            }
        },
        common::Variant::MoveFar { who, to } => {
            
        },
    }
}

fn finish_round_command(command:RoundCommand, round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>, grid:&mut Grid) {
    match command.variant {
        common::Variant::Nop => {},
        common::Variant::MoveTo { who, to } => {
            if let Ok(mut token) = tokens.get_mut(who) {
                token.grid_pos = to;
                if let Ok(mut transform) = transforms.get_mut(who) {
                    transform.translation = Token::pos(token.grid_pos);
                }
            }
        },
        common::Variant::MoveFar { who, to } => {
            if let Ok(token) = tokens.get(who) {
                let path = rules::get_path(token, &grid, to);
                if path.len() > 0 {
                    for p in path {
                        round.push_back_command(RoundCommand::move_to(who, p.to));
                    }
                }
            }
        },
    }
}

fn update_round(mut round:ResMut<Round>, time:Res<Time>, mut tokens:Query<&mut Token>, mut transforms:Query<&mut Transform>, mut grid:ResMut<Grid>) {
    if let Some(mut command) = round.pop_front() {
        command.timer_elapsed_sec += time.delta_seconds();
        command.timer_elapsed_sec = command.timer_elapsed_sec.min(command.timer);
        update_round_command(&mut command, &mut round, &time, &mut tokens, &mut transforms);
        if command.timer_elapsed_sec >= command.timer {
            finish_round_command(command, &mut round, &time, &mut tokens, &mut transforms, &mut grid);
        } else {
            round.push_front_command(command);
        }
    }
}


pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, update_round);
        app.add_systems(PostUpdate, spawned_token);
    }
}
