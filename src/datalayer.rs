use crate::world::DataWorld;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::{EntityWorldMut, IntoSystem};
use bevy_ecs::system::BoxedSystem;
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
    fn run<S, I, R, T>(&mut self, system: S, input: I) -> R
    where
        S: IntoSystem<I, R, T>,
        R: 'static,
        I: 'static,
    {
        let mut boxed_system: BoxedSystem<I, R> = Box::new(IntoSystem::into_system(system));
        self.run_boxed(&mut boxed_system, input)
    }

    fn run_boxed<R: 'static, I: 'static>(&mut self, system: &mut BoxedSystem<I, R>, input: I) -> R {
        system.initialize(&mut self.world);
        let to_return = system.run(input, &mut self.world);
        system.apply_deferred(&mut self.world);

        to_return
    }

    fn get_resource<R: prelude::Resource + Clone>(&self) -> Option<R> {
        self.world.get_resource::<R>().cloned()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }
}
