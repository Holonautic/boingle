use crate::gadgets::systems::on_finish_easing_destroy;
use bevy::color::palettes::tailwind;
use bevy::ecs::component::HookContext;
use bevy::ecs::world;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use bevy_easings::{Ease, EaseFunction, EaseMethod, EasingType};
use std::time::Duration;
use bevy::color::palettes::basic::WHITE;

#[derive(Component, Reflect, Clone, Debug)]
pub struct UiPointsText;

#[derive(Component, Reflect, Clone, Debug)]
pub struct UiCoinsText;

#[derive(Component)]
pub struct UiLevelOver;

#[derive(Component)]
pub struct UiBallsText;
#[derive(Component)]
pub struct UiCurrentLevelText;
#[derive(Component)]
pub struct UiPointsForNextLevel;

#[derive(Component)]
pub struct UiWidgetSelectionRoot;

#[derive(Component)]
#[component(on_add=FloatingScore::on_add)]
pub struct FloatingScore(pub usize);

impl FloatingScore {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let score = world.get::<FloatingScore>(context.entity).unwrap().0;
        let transform = world.get::<Transform>(context.entity).unwrap();
        let easing = transform.ease_to_fn(
            |start| Transform {
                translation: start.translation + Vec3::Y * 100.0,
                scale: start.scale * 0.3,
                ..*start
            },
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: Duration::from_secs_f32(1.5),
            },
        );

        world.commands().entity(context.entity).insert((
            Text2d(format!("{}", score)),
            TextColor(tailwind::GRAY_800.into())
                .ease_to(
                    TextColor(tailwind::GRAY_100.into()),
                    EaseMethod::Linear,
                    EasingType::Once {
                        duration: Duration::from_secs_f32(1.0),
                    },
                )
                .with_original_value(),
            easing,
            observers![on_finish_easing_destroy],
        ));
    }
}
