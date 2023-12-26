#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "ron")]
pub mod ron;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In};

pub fn mark_with<C: Component + Default>(In(entities): In<Vec<Entity>>, mut commands: Commands) {
    for entity in entities {
        commands.entity(entity).insert(C::default());
    }
}
