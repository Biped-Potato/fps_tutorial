use bevy::prelude::*;

#[derive(Resource,Default)]
pub struct PlayerInput{
    //x component is forward and y direction is right
    pub movement : Vec2,
}