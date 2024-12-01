use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::{EntityWorldMut, SystemInput};
use bevy_ecs::system::{IntoSystem, Resource};

/// Access to the inner [`bevy_ecs::world::World`] of the datalayer.
pub trait DataWorld {
    /// Run a given system with inputs against the [`bevy_ecs::world::World`].
    fn run<S, In, Out, Marker>(&mut self, system: S, input: In::Inner<'_>) -> Out
    where
        S: IntoSystem<In, Out, Marker> + 'static,
        Out: 'static,
        In: SystemInput + 'static;

    /// Get a [`Resource`] form the [`bevy_ecs::world::World`].
    fn get_resource<R: Resource + Clone>(&self) -> Option<R>;

    /// Spawn a new entity into the [`bevy_ecs::world::World`].
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;
}
