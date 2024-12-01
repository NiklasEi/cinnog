use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::{EntityWorldMut, SystemInput};
use bevy_ecs::system::IntoSystem;

pub trait DataWorld {
    fn run<S, In, Out, Marker>(&mut self, system: S, input: In::Inner<'_>) -> Out
    where
        S: IntoSystem<In, Out, Marker> + 'static,
        Out: 'static,
        In: SystemInput + 'static;

    fn get_resource<R: prelude::Resource + Clone>(&self) -> Option<R>;

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;
}
