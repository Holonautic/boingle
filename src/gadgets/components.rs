use std::fmt::{Display, Formatter};
use bevy::prelude::*;
use crate::gadgets::{bumper_bundle, coin_bumper_bundle, large_block_bundle};
use crate::gadgets::resources::GadgetImageResource;

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
pub enum GadgetType {
    LargeBlock,
    Bumper,
    CoinBumper,
    // Cannon,
}

impl GadgetType {
    pub fn card_icon_size(&self) -> Vec3 {
        match self {
            GadgetType::LargeBlock => Vec3::splat(0.3),
            GadgetType::Bumper => Vec3::splat(1.0),
            GadgetType::CoinBumper => Vec3::splat(0.5),
        }
    }

    pub fn spawn_widget(&self, commands: &mut Commands, asset_resource: &GadgetImageResource) -> Entity {
        match self {
            GadgetType::LargeBlock => commands.spawn(large_block_bundle(asset_resource)).id(),
            GadgetType::Bumper => commands.spawn(bumper_bundle(asset_resource)).id(),
            GadgetType::CoinBumper => commands.spawn(coin_bumper_bundle(asset_resource)).id()
        }
    }
}

impl Display for GadgetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GadgetType::LargeBlock => write!(f,"Large Block"),
            GadgetType::Bumper => write!(f, "Bumper"),
            GadgetType::CoinBumper => write!(f, "Coin Bumper"),
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Preview;

#[derive(Component, Debug, Reflect)]
pub struct PlayerPlacedGadget;

pub struct Gadget {
    pub activations_left: usize,
    pub activations: usize,
}


