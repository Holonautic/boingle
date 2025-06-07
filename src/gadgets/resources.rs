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
        
        self.gadget_images.insert(GadgetType::Bumper, asset_server.load("sprites/bumper_points.png"));
        self.gadget_images.insert(GadgetType::SquareBlock, asset_server.load("sprites/block_square.png"));
        self.gadget_images.insert(GadgetType::WideBlock, asset_server.load("sprites/block_square.png"));

        self.gadget_images.insert(GadgetType::CoinBumper, asset_server.load("sprites/bumper_coins.png"));
        self.gadget_images.insert(GadgetType::BallCannon, asset_server.load("sprites/cannon.png"));

        self.collectibles_images.insert(CollectibleType::Coin, asset_server.load("sprites/coin.png"));

        self.ball_image = asset_server.load("sprites/ball_1.png");
    }

    pub fn card_sprite(&self, gadget_type: &GadgetType) -> Sprite {
        let slice_border = 30.0;
        let image_mode = SpriteImageMode::Sliced(TextureSlicer {
            border: BorderRect::all(slice_border),
            center_scale_mode: SliceScaleMode::Stretch,
            ..default()
        });
        let default_size = 50.0;
        let image = self.gadget_images[gadget_type].clone();
        match gadget_type {
            GadgetType::SquareBlock => {
                Sprite {
                    image,
                    image_mode,
                    custom_size: Some(Vec2::new(default_size, default_size)),
                    ..default()
                }
            }
            GadgetType::WideBlock => {
                Sprite {
                    image,
                    image_mode,
                    custom_size: Some(Vec2::new(80.0, 40.0)),

                    ..default()
                }
            }
            GadgetType::Bumper => {
                Sprite {
                    image,
                    custom_size: Some(Vec2::new(default_size, default_size)),

                    ..default()
                }
            }
            GadgetType::CoinBumper => {
                Sprite {
                    image,
                    custom_size: Some(Vec2::new(default_size, default_size)),

                    ..default()
                }
            }
            GadgetType::BallCannon => {
                Sprite {
                    image,
                    ..default()
                }
            }
        }

    }

}