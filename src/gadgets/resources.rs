use std::collections::HashMap;
use bevy::prelude::*;
use crate::gadgets::components::GadgetType;

#[derive(Resource, Default)]
pub struct GadgetImageResource {
    pub gadget_images: HashMap<GadgetType, Handle<Image>>,
}

impl GadgetImageResource {
    pub fn setup(&mut self, asset_server: &AssetServer) {
        self.gadget_images.insert(GadgetType::Bumper, asset_server.load("sprites/hole.png"));
        self.gadget_images.insert(GadgetType::LargeBlock, asset_server.load("sprites/block_large.png"));
        self.gadget_images.insert(GadgetType::CoinBumper, asset_server.load("sprites/hole_large_end_locked.png"));

    }
}