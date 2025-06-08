use crate::cards::components::{ShopCard, ShopCardType};
use bevy::ecs::component::HookContext;
use bevy::prelude::*;
use bevy_bundled_observers::bevy_ecs::world::DeferredWorld;
use bevy_bundled_observers::observers;

#[derive(Component, Event, Reflect, Debug)]
#[component(on_add=OnGadgetCardSelected::on_add)]
pub struct OnGadgetCardSelected {
    pub shop_card_type: ShopCardType,
}

impl OnGadgetCardSelected {
    pub fn new(shop_card_type: ShopCardType) -> Self {
        Self { shop_card_type }
    }

    fn on_add(mut world: DeferredWorld, context: HookContext) {
        world
            .commands()
            .entity(context.entity)
            .insert(observers![Self::on_selected]);
    }

    pub fn on_selected(
        trigger: Trigger<Pointer<Click>>,
        mut commands: Commands,
        q_card: Query<&ShopCard>,
    ) {
        let shop_card = q_card.get(trigger.target).unwrap();
        commands.trigger_targets(
            OnGadgetCardSelected::new(shop_card.card_type.clone()),
            trigger.target,
        );
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnCoinCollected;
