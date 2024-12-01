#[cfg(feature = "generator")]
use crate::generator::Generator;
use crate::Ingest;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Commands, Resource, SystemSet};
use bevy_ecs::schedule::IntoSystemConfigs;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::marker::PhantomData;

/// System sets for ron related systems
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RonSystems {
    /// System set containing systems that read ron files
    Read,
}

/// Extension trait for the data layer to add ron specific methods
pub trait RonDataLayer {
    /// Add a directory to be loaded as a collection fo ron files
    fn add_ron_directory<R: Ingest + DeserializeOwned + Sync + Send + 'static>(
        &mut self,
        directory: impl Into<String>,
    ) -> &mut Self;
}

#[cfg(feature = "generator")]
impl RonDataLayer for Generator {
    fn add_ron_directory<R: Ingest + DeserializeOwned + Sync + Send + 'static>(
        &mut self,
        directory: impl Into<String>,
    ) -> &mut Self {
        self.app.init_resource::<RonDirectories<R>>();
        if !self.app.is_plugin_added::<ReadRon<R>>() {
            self.add_plugins(ReadRon::<R>::new());
        }

        let mut directories = self.app.world_mut().resource_mut::<RonDirectories<R>>();
        directories.directories.push(directory.into());

        self
    }
}

struct ReadRon<M: Ingest + DeserializeOwned + Sync + Send + 'static> {
    _marker: PhantomData<M>,
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> Plugin for ReadRon<R> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            read_ron_files_from_directory::<R>.in_set(RonSystems::Read),
        );
    }
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> ReadRon<R> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

#[derive(Resource)]
struct RonDirectories<R: Ingest + DeserializeOwned + Sync + Send + 'static> {
    directories: Vec<String>,
    _marker: PhantomData<R>,
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> Default for RonDirectories<R> {
    fn default() -> Self {
        Self {
            directories: vec![],
            _marker: PhantomData,
        }
    }
}

fn read_ron_files_from_directory<R: Ingest + DeserializeOwned + Sync + Send + 'static>(
    mut commands: Commands,
    directories: Res<RonDirectories<R>>,
) {
    for directory in &directories.directories {
        for entry in fs::read_dir(directory).expect("Failed to read directory") {
            let entry = entry.expect("Failed read directory entry");
            let path = entry.path();
            if path.is_file() {
                let file = File::open(path.clone()).expect("Failed to open file");
                let data: R = ron::de::from_reader(file).expect("Failed to parse ron");
                let mut entity = commands.spawn(());
                data.ingest_path(&mut entity, &path);
                data.ingest(&mut entity);
            }
        }
    }
}
