#![forbid(unsafe_code)]

mod datalayer;
#[cfg(feature = "generator")]
pub mod generator;
pub mod loaders;
mod world;

use crate::datalayer::Datalayer;
use crate::world::DataWorld;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::system::{EntityCommands, IntoSystem, Resource};
use leptos::prelude::expect_context;
use std::any::type_name;
use std::path::Path;
use std::sync::{Arc, Mutex};

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
    let cinnog = expect_context::<Arc<Mutex<Datalayer>>>();
    let mut data_layer = cinnog.lock().unwrap();

    data_layer.run(system, input)
}

pub fn use_resource<R: Resource + Clone>() -> Option<R> {
    let cinnog = expect_context::<Arc<Mutex<Datalayer>>>();
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
