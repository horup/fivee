use std::collections::VecDeque;
use bevy::prelude::{Resource, Entity};
use glam::IVec2;

pub enum Variant {
    Nop,
    MoveTo {
        who:Entity,
        to:IVec2
    }
}

impl Default for Variant {
    fn default() -> Self {
        Self::Nop
    }
}

#[derive(Default)]
pub struct RoundCommand {
    pub timer:f32,
    pub timer_elapsed_sec:f32,
    pub parallel:bool,
    pub variant:Variant,
}

impl RoundCommand {
    pub fn nop() -> Self {
        Self {
            timer:1.0,
            variant:Variant::Nop,
            ..Default::default()
        }
    }

    pub fn move_to(who:Entity, to:IVec2) -> Self {
        Self {
            timer:2.5,
            variant:Variant::MoveTo { who, to},
            ..Default::default()
        }
    }

    pub fn alpha(&self) -> f32 {
        if self.timer == 0.0 {
            return 1.0;
        }
        let a = self.timer_elapsed_sec / self.timer;
        if a < 0.0 {
            return 0.0;
        } else if a > 1.0 {
            return a;
        }
        
        return a;
    }
}

#[derive(Resource, Default)]
pub struct Round {
    commands:VecDeque<RoundCommand>
}

impl Round {
    pub fn push_front_command(&mut self, command:RoundCommand) {
        self.commands.push_front(command);
    }

    pub fn push_back_command(&mut self, command:RoundCommand) {
        self.commands.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<RoundCommand> {
        self.commands.pop_front()
    }

    pub fn executing(&self) -> bool {
        self.commands.len() > 0
    }
}