use crate::cards::components::*;
use crate::gadgets::components::*;
use crate::gameplay::components::GadgetCard;
use bevy::prelude::*;
use bevy_rand::prelude::{Entropy, WyRand};
use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet};
use rand::Rng;

#[derive(Resource, Default)]
pub struct GameResources {
    pub gadget_images: HashMap<GadgetType, Handle<Image>>,
    pub collectibles_images: HashMap<CollectibleType, Handle<Image>>,
    pub play_area: Vec2,
    pub ball_image: Handle<Image>,

    pub balls_per_level: usize,
    pub gadgets: HashMap<ShopCardType, Gadget>,
    pub gadget_points: HashMap<ShopCardType, PointsOnHit>,
    pub activation_time: HashMap<ShopCardType, ActivationTime>,

    shop_items_per_level: Vec<Vec<ShopCardType>>,
}

impl GameResources {
    pub fn setup(&mut self, asset_server: &AssetServer) {
        self.play_area = Vec2::new(450.0, 250.0);

        self.gadget_images.insert(
            GadgetType::Bumper,
            asset_server.load("sprites/bumper_points.png"),
        );
        self.gadget_images.insert(
            GadgetType::SquareBlock,
            asset_server.load("sprites/block_square.png"),
        );
        self.gadget_images.insert(
            GadgetType::WideBlock,
            asset_server.load("sprites/block_square.png"),
        );

        self.gadget_images.insert(
            GadgetType::CoinBumper,
            asset_server.load("sprites/bumper_coins.png"),
        );
        self.gadget_images.insert(
            GadgetType::BallCannon,
            asset_server.load("sprites/cannon.png"),
        );

        self.collectibles_images
            .insert(CollectibleType::CoinType, asset_server.load("sprites/coin.png"));

        self.ball_image = asset_server.load("sprites/ball_1.png");

        self.fill_gadgets();
        self.fill_points_for_card();
        self.fill_activation_time_for_card();
        self.fill_shop_items();
        self.balls_per_level = 3;
    }

    pub fn get_random_position_in_play_area(&self, rng:&mut  Entropy<WyRand>) -> Vec2 {
        let x = rng.random_range(-self.play_area.x..self.play_area.x);
        let y = rng.random_range(-self.play_area.y..self.play_area.y);
        Vec2::new(x, y)
    }
    pub fn card_title(&self, shop_card_type: &ShopCardType) -> String {
        match shop_card_type {
            ShopCardType::OneMoreBallCard => "+1 Ball".to_string(),
            ShopCardType::MoreBallsCard => format!("+{} Ball", self.balls_per_level),
            ShopCardType::SquareBlockCard => "Square Block".to_string(),
            ShopCardType::WideBlockCard => "Wide Block".to_string(),
            ShopCardType::BumperCard => "Bumper".to_string(),
            ShopCardType::CoinBumperCard => "CoinBumper".to_string(),
            ShopCardType::HighFrictionBlockCard => "High Friction Block".to_string(),
            ShopCardType::MagnetCard => "Magnetise".to_string(),
            ShopCardType::ReactivateLaserBridgeCard => "Reactivate".to_string(),
            ShopCardType::GravityReverserCard => "Gravity Reverser".to_string(),
            ShopCardType::MultiBallCard => "Multi Ball".to_string(),
            ShopCardType::RecycleGadgetCard => "Recycle Gadget".to_string(),
            ShopCardType::RearrangeGadgetCard => "Rearrange Gadget".to_string(),
        }
    }

    pub fn fill_gadgets(&mut self) {
        self.gadgets
            .insert(ShopCardType::SquareBlockCard, Gadget::new(5));
        self.gadgets
            .insert(ShopCardType::WideBlockCard, Gadget::new(5));
        self.gadgets
            .insert(ShopCardType::BumperCard, Gadget::new(3));
        self.gadgets
            .insert(ShopCardType::CoinBumperCard, Gadget::new(1));
        self.gadgets
            .insert(ShopCardType::MultiBallCard, Gadget::new(1));
    }

    pub fn fill_points_for_card(&mut self) {
        self.gadget_points
            .insert(ShopCardType::SquareBlockCard, PointsOnHit::new(1));
        self.gadget_points
            .insert(ShopCardType::WideBlockCard, PointsOnHit::new(1));
        self.gadget_points
            .insert(ShopCardType::BumperCard, PointsOnHit::new(3));
    }

    pub fn fill_activation_time_for_card(&mut self) {
        self.activation_time
            .insert(ShopCardType::MagnetCard, ActivationTime::new(5.0));
    }

    pub fn card_description(&self, shop_card_type: &ShopCardType) -> String {
        match shop_card_type {
            ShopCardType::OneMoreBallCard => "Adds 1 Ball".to_string(),
            ShopCardType::MoreBallsCard => format!("Adds {} Balls", self.balls_per_level),
            ShopCardType::SquareBlockCard => format!(
                "{}x{} Points",
                self.gadget_points.get(shop_card_type).unwrap().amount,
                self.gadgets
                    .get(shop_card_type)
                    .unwrap()
                    .activations_per_round
            ),
            ShopCardType::WideBlockCard => format!(
                "{}x{} Points",
                self.gadget_points.get(shop_card_type).unwrap().amount,
                self.gadgets
                    .get(shop_card_type)
                    .unwrap()
                    .activations_per_round
            ),
            ShopCardType::BumperCard => format!(
                "{}x{} Points",
                self.gadget_points.get(shop_card_type).unwrap().amount,
                self.gadgets
                    .get(shop_card_type)
                    .unwrap()
                    .activations_per_round
            ),
            ShopCardType::CoinBumperCard => "Spawn Coins".to_string(),
            ShopCardType::HighFrictionBlockCard => "Slows down ball".to_string(),
            ShopCardType::MagnetCard => format!(
                "Attracts coins for {:.1}s",
                self.activation_time
                    .get(shop_card_type)
                    .unwrap()
                    .time
                    .as_secs_f32()
            ),
            ShopCardType::ReactivateLaserBridgeCard => "Reactivate Gadgets".to_string(),
            ShopCardType::GravityReverserCard => "Reverse Gravity in field".to_string(),
            ShopCardType::MultiBallCard => "Duplicate Ball".to_string(),
            ShopCardType::RecycleGadgetCard => "Recycle a Gadget for Coins".to_string(),
            ShopCardType::RearrangeGadgetCard => "Move an already placed Gadget".to_string(),
        }
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
            GadgetType::SquareBlock => Sprite {
                image,
                image_mode,
                custom_size: Some(Vec2::new(default_size, default_size)),
                ..default()
            },
            GadgetType::WideBlock => Sprite {
                image,
                image_mode,
                custom_size: Some(Vec2::new(80.0, 40.0)),

                ..default()
            },
            GadgetType::Bumper => Sprite {
                image,
                custom_size: Some(Vec2::new(default_size, default_size)),

                ..default()
            },
            GadgetType::CoinBumper => Sprite {
                image,
                custom_size: Some(Vec2::new(default_size, default_size)),

                ..default()
            },
            GadgetType::BallCannon => Sprite { image, ..default() },
        }
    }

    pub fn get_shop_cards_for_level(
        &self,
        level: usize,
        rng: &mut Entropy<WyRand>,
    ) -> Vec<ShopCardType> {
        let mut cards = vec![ShopCardType::MoreBallsCard];

        let safe_level = level.min(self.shop_items_per_level.len() - 1);

        for card in self.shop_items_per_level[safe_level]
            .iter()
            .choose_multiple(rng, 3)
        {
            cards.push(*card);
        }

        cards
    }
    pub fn fill_shop_items(&mut self) {


        self.shop_items_per_level.push(vec![
            ShopCardType::OneMoreBallCard,
            ShopCardType::BumperCard,
            ShopCardType::CoinBumperCard,
        ]);
        self.shop_items_per_level.push(vec![
            ShopCardType::OneMoreBallCard,
            ShopCardType::BumperCard,
            ShopCardType::CoinBumperCard,
            ShopCardType::WideBlockCard,
        ]);
        self.shop_items_per_level.push(vec![
            ShopCardType::OneMoreBallCard,
            ShopCardType::BumperCard,
            ShopCardType::CoinBumperCard,
            ShopCardType::WideBlockCard,
        ]);
    }

    pub fn get_price_per_card(&self, shop_card_type: &ShopCardType) -> usize {
        match shop_card_type {
            ShopCardType::OneMoreBallCard => 3,
            ShopCardType::MoreBallsCard => 0,
            ShopCardType::SquareBlockCard => 1,
            ShopCardType::WideBlockCard => 1,
            ShopCardType::BumperCard => 2,
            ShopCardType::CoinBumperCard => 5,
            ShopCardType::HighFrictionBlockCard => 4,
            ShopCardType::MagnetCard => 9,
            ShopCardType::ReactivateLaserBridgeCard => 25,
            ShopCardType::GravityReverserCard => 12,
            ShopCardType::MultiBallCard => 35,
            ShopCardType::RecycleGadgetCard => 15,
            ShopCardType::RearrangeGadgetCard => 8,
        }
    }
}
