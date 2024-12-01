#![forbid(unsafe_code)]
#![deny(missing_docs)]
///! Cinnog is an experimental static site generator using [Leptos](https://www.leptos.dev/)
///!
///! ## Datalayer
///! Cinnog uses Bevy ECS as [`Datalayer`] to simplify site data preparation and handling.
///! Before building the site, you can add any data into the datalayer or manipulate already
///! exising data.
///! During site generation, your Leptos components can query the data layer and generate
///! content based on it. Example workflows are automatic conversion of markdown files to HTML,
///! or resizing all media content.
mod datalayer;
#[cfg(feature = "generator")]
pub mod generator;
pub mod loaders;
mod world;

use crate::datalayer::Datalayer;
use crate::world::DataWorld;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::system::{EntityCommands, IntoSystem, Resource, SystemInput};
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
    S: IntoSystem<(), R, T> + 'static,
    R: 'static,
{
    run_system_with_input(system, ())
}

pub fn run_system_with_input<S, I, R, T>(system: S, input: I::Inner<'_>) -> R
where
    S: IntoSystem<I, R, T> + 'static,
    R: 'static,
    I: SystemInput + 'static,
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

/// Default components marking an entity as representing a file on the file system with a given
/// [`Path`].
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
