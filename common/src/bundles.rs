use bevy::prelude::{Bundle, Handle};
use crate::{Token, Statblock};

#[derive(Bundle)]
pub struct TokenBundle {
    pub token:Token,
    pub statblock:Handle<Statblock>
}