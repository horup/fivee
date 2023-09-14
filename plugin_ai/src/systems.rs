use crate::components::AI;
use bevy::prelude::*;
use common::{Round, RoundCommand, Token};

fn add_remove_ai_system(mut commands: Commands, tokens: Query<(Entity, &Token)>, ais: Query<&AI>) {
    for (token_entity, token) in tokens.iter() {
        if token.player == None {
            if !ais.contains(token_entity) {
                commands.entity(token_entity).insert(AI::default());
            }
        } else {
            if ais.contains(token_entity) {
                commands.entity(token_entity).remove::<AI>();
            }
        }
    }
}

fn timeout_system(mut round: ResMut<Round>, mut ais: Query<&mut AI, With<Token>>, time: Res<Time>) {
    if round.is_executing() {
        return;
    }

    let Some(active_token) = round.active_token else {
        return;
    };

    let Ok(mut active_ai) = ais.get_mut(active_token) else {
        return;
    };

    active_ai.timeout_timer += time.delta_seconds();

    if active_ai.timeout_timer > 1.0 {
        round.push_back_command(RoundCommand::give_turn(active_token));
        active_ai.timeout_timer = 0.0;
    }
}

fn think_system(
    mut round: ResMut<Round>,
    mut ais: Query<&mut AI, With<Token>>,
    tokens: Query<&Token>,
) {
    if round.is_executing() {
        return;
    }

    let Some(entity) = round.active_token else {
        return;
    };

    let Ok(mut active_ai) = ais.get_mut(entity) else {
        return;
    };

    let Ok(token) = tokens.get(entity) else {
        return;
    };

    let new_pos = token.grid_pos + IVec2::new(1, 0);
    round.push_back_command(RoundCommand::move_far(entity, new_pos));
    round.push_back_command(RoundCommand::give_turn(entity));
}

pub fn add_systems(app: &mut App) {
    app.add_systems(Update, (add_remove_ai_system, timeout_system, think_system));
}
