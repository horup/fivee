

use bevy::prelude::*;
use common::{CommonAssets, Token, Round, RoundCommand};



mod startup;
pub use startup::*;


fn spawned_token(mut commands:Commands, q:Query<(Entity, &Token), Added<Token>>, sa:Res<CommonAssets>, mut materials:ResMut<Assets<StandardMaterial>>) {
    for (e, token) in q.iter() {
        commands.entity(e).insert(PbrBundle {
            transform:Transform::from_translation(Token::pos(token.grid_pos)),
            mesh:sa.mesh("token"),
            material:materials.add(StandardMaterial {
                base_color:token.color.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

fn update_round_command(command:&mut RoundCommand, _round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
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
                    transform.translation = s + v;
                }
            }
        },
    }
}

fn finish_round_command(command:RoundCommand, _round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
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
    }
}

fn update_round(mut round:ResMut<Round>, time:Res<Time>, mut tokens:Query<&mut Token>, mut transforms:Query<&mut Transform>) {
    if let Some(mut command) = round.pop_front() {
        command.timer_elapsed_sec += time.delta_seconds();
        command.timer_elapsed_sec = command.timer_elapsed_sec.min(command.timer);
        update_round_command(&mut command, &mut round, &time, &mut tokens, &mut transforms);
        if command.timer_elapsed_sec >= command.timer {
            finish_round_command(command, &mut round, &time, &mut tokens, &mut transforms);
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
