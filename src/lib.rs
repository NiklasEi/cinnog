#[cfg(feature = "generator")]
pub mod generator;
pub mod loaders;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::system::{BoxedSystem, IntoSystem, Resource};
use bevy_ecs::world::{EntityWorldMut, World};
use leptos::expect_context;
use std::any::type_name;
use std::path::PathBuf;
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

    pub fn run_boxed<R: 'static, I: 'static>(
        &mut self,
        system: &mut BoxedSystem<I, R>,
        input: I,
    ) -> R {
        system.initialize(&mut self.world);
        let to_return = system.run(input, &mut self.world);
        system.apply_deferred(&mut self.world);

        to_return
    }

    pub fn run<S, I, R, T>(&mut self, system: S, input: I) -> R
    where
        S: IntoSystem<I, R, T>,
        R: 'static,
        I: 'static,
    {
        let mut boxed_system: BoxedSystem<I, R> = Box::new(IntoSystem::into_system(system));
        self.run_boxed(&mut boxed_system, input)
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
    run_system_with_input(system, ())
}

pub fn run_system_with_input<S, I, R, T>(system: S, input: I) -> R
where
    S: IntoSystem<I, R, T>,
    R: 'static,
    I: 'static,
{
    let cinnog = expect_context::<Arc<Mutex<DataLayer>>>();
    let mut lock = cinnog.lock().unwrap();

    lock.run(system, input)
}

pub fn use_resource<R: Resource + Clone>() -> Option<R> {
    let cinnog = expect_context::<Arc<Mutex<DataLayer>>>();
    let lock = cinnog.lock().unwrap();
    lock.get_resource()
}

#[derive(Component, Clone)]
pub struct FileName(pub String);

#[derive(Component, Clone)]
pub struct FilePath(pub String);

pub trait ToBundle {
    fn to_bundle(self) -> impl Bundle
    where
        Self: Sized;

    fn to_bundle_with_path(self, path: &PathBuf) -> impl Bundle
    where
        Self: Sized,
    {
        let path_string = path.to_string_lossy().into_owned();
        let file_ending = path.as_path().extension();
        let file_name = if let Some(ending) = file_ending {
            path.file_name().map(|name| {
                name.to_string_lossy()
                    .trim_end_matches(&format!(".{}", ending.to_string_lossy().as_ref()))
                    .to_owned()
            })
        } else {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        };
        (
            self.to_bundle(),
            FileName(file_name.expect("No file name in path")),
            FilePath(path_string),
        )
    }
}
