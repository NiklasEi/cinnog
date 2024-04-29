#[cfg(feature = "generator")]
pub mod generator;
pub mod loaders;

use bevy_app::{App, Plugins};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::system::{BoxedSystem, EntityCommands, IntoSystem, Resource};
use bevy_ecs::world::EntityWorldMut;
use leptos::expect_context;
use std::any::type_name;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct DataLayer {
    app: App,
}

impl DataLayer {
    pub fn new() -> Self {
        DataLayer { app: App::new() }
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) -> &mut Self {
        self.app.insert_resource(value);
        self
    }

    pub fn get_resource<R: Resource + Clone>(&self) -> Option<R> {
        self.app.world.get_resource::<R>().cloned()
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.app.world.spawn(bundle)
    }

    pub fn run_boxed<R: 'static, I: 'static>(
        &mut self,
        system: &mut BoxedSystem<I, R>,
        input: I,
    ) -> R {
        system.initialize(&mut self.app.world);
        let to_return = system.run(input, &mut self.app.world);
        system.apply_deferred(&mut self.app.world);

        to_return
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        self.app.add_plugins(plugins);
        self
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
    let mut data_layer = cinnog.lock().unwrap();

    data_layer.run(system, input)
}

pub fn use_resource<R: Resource + Clone>() -> Option<R> {
    let cinnog = expect_context::<Arc<Mutex<DataLayer>>>();
    let lock = cinnog.lock().unwrap();
    lock.get_resource()
}

#[derive(Component, Clone, Debug)]
pub struct FileName(pub String);

#[derive(Component, Clone, Debug)]
pub struct StaticPath(pub String);

#[derive(Component, Clone, Debug)]
pub struct FilePath(pub String);

pub trait Ingest {
    fn ingest(self, commands: &mut EntityCommands)
    where
        Self: Sized;

    fn ingest_path(&self, commands: &mut EntityCommands, path: &Path) {
        commands.insert(default_bundle_from_path(path));
    }
}

pub fn default_bundle_from_path(path: &Path) -> impl Bundle {
    let path_string = path.to_string_lossy().into_owned();
    let file_ending = path.extension();
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
        FileName(file_name.expect("No file name in path")),
        FilePath(path_string),
    )
}
