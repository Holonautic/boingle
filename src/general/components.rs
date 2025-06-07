use std::ops::Deref;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct MainCamera;

#[derive(Component, Reflect, Default, Debug)]
#[component(on_add=on_add_test)]
pub struct TestHookComponent {
    pub some_text: String,
}

fn on_add_test(world: DeferredWorld,
               context: HookContext) {
    let thing = world.get::<TestHookComponent>(context.entity).unwrap();
    info!("well it seems the hook worked {}",thing.some_text);
}

#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = SpriteVisual)]
pub struct SpriteVisualOf(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = SpriteVisualOf)]
pub struct SpriteVisual(Vec<Entity>);

impl Deref for SpriteVisual {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0[0]
    }
}