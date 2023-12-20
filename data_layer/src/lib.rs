#![feature(async_closure)]

#[cfg(feature = "generator")]
pub mod generator;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::system::{BoxedSystem, IntoSystem, Resource};
use bevy_ecs::world::{EntityWorldMut, World};
use leptos::expect_context;
use std::any::type_name;
use std::sync::{Arc, Mutex};

pub struct DataLayer {
    world: World,
}

impl DataLayer {
    pub fn new() -> Self {
        DataLayer {
            world: World::new(),
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) {
        self.world.insert_resource(value)
    }

    pub fn get_resource<R: Resource + Clone>(&self) -> Option<R> {
        self.world.get_resource::<R>().cloned()
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }

    pub fn run_boxed<R: 'static>(&mut self, system: &mut BoxedSystem<(), R>) -> R {
        system.initialize(&mut self.world);
        let to_return = system.run((), &mut self.world);
        system.apply_deferred(&mut self.world);

        to_return
    }

    pub fn run<S, R, T>(&mut self, system: S) -> R
    where
        S: IntoSystem<(), R, T>,
        R: 'static,
    {
        let mut boxed_system: BoxedSystem<(), R> = Box::new(IntoSystem::into_system(system));
        self.run_boxed(&mut boxed_system)
    }
}

impl Default for DataLayer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn expect_resource<R: Resource + Clone>() -> R {
    use_resource::<R>().unwrap_or_else(|| {
        panic!(
            "Expected resource {}, but it didn't exist",
            type_name::<R>()
        )
    })
}

pub fn run_system<S, R, T>(system: S) -> R
where
    S: IntoSystem<(), R, T>,
    R: 'static,
{
    let data_layer = expect_context::<Arc<Mutex<DataLayer>>>();
    let mut lock = data_layer.lock().unwrap();

    lock.run(system)
}

pub fn use_resource<R: Resource + Clone>() -> Option<R> {
    let data_layer = expect_context::<Arc<Mutex<DataLayer>>>();
    let lock = data_layer.lock().unwrap();
    lock.get_resource()
}
