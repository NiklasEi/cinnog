/// Functionality related to markdown and conversions from it
#[cfg(feature = "markdown")]
pub mod markdown;
/// Functionality related to the file format ron
#[cfg(feature = "ron")]
pub mod ron;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In};

/// Add a component to a list of entities
///
/// This is a convenience method to add components with default values to multiple entities.
pub fn mark_with<C: Component + Default>(In(entities): In<Vec<Entity>>, mut commands: Commands) {
    for entity in entities {
        commands.entity(entity).insert(C::default());
    }
}
