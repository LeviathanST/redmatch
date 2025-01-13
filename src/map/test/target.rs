use bevy::{
    math::Vec3,
    prelude::{Entity, Query, Res, Resource, Transform, With},
    time::Time,
};
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::player::Player;

use super::Ground;
