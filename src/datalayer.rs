use crate::world::DataWorld;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::{EntityWorldMut, IntoSystem};
use bevy_ecs::system::SystemInput;
use bevy_ecs::world::World;

pub struct Datalayer {
    world: World,
}

impl Datalayer {
    pub fn new(world: World) -> Datalayer {
        Datalayer { world }
    }
}

impl DataWorld for Datalayer {
    fn run<S, In, Out, Marker>(&mut self, system: S, input: In::Inner<'_>) -> Out
    where
        S: IntoSystem<In, Out, Marker> + 'static,
        Out: 'static,
        In: SystemInput + 'static,
    {
        self.world
            .run_system_cached_with(system, input)
            .expect("Failed to execute system")
    }

    fn get_resource<R: prelude::Resource + Clone>(&self) -> Option<R> {
        self.world.get_resource::<R>().cloned()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }
}
