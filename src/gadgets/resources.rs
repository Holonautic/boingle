use std::collections::HashMap;
use bevy::prelude::*;
use crate::gadgets::components::{CollectibleType, GadgetType};

#[derive(Resource, Default)]
pub struct GameResources {
    pub gadget_images: HashMap<GadgetType, Handle<Image>>,
    pub collectibles_images: HashMap<CollectibleType, Handle<Image>>,
    pub play_area: Vec2,
    
    pub ball_image: Handle<Image>,
}

impl GameResources {
    pub fn setup(&mut self, asset_server: &AssetServer) {
        self.play_area = Vec2::new(450.0, 250.0);
        
        self.gadget_images.insert(GadgetType::Bumper, asset_server.load("sprites/hole.png"));
        self.gadget_images.insert(GadgetType::LargeBlock, asset_server.load("sprites/block_large.png"));
        self.gadget_images.insert(GadgetType::CoinBumper, asset_server.load("sprites/hole_large_end_locked.png"));
        self.gadget_images.insert(GadgetType::BallCannon, asset_server.load("sprites/cannon.png"));

        self.collectibles_images.insert(CollectibleType::Coin, asset_server.load("sprites/star.png"));
        
        self.ball_image = asset_server.load("sprites/ball_1.png");
    }

}