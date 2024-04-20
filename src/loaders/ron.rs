use crate::Ingest;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Commands, Resource, SystemSet};
use bevy_ecs::schedule::IntoSystemConfigs;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::marker::PhantomData;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RonSystems {
    Read,
}

pub struct ReadRonDirectory<M: Ingest + DeserializeOwned + Sync + Send + 'static> {
    directory: String,
    _marker: PhantomData<M>,
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> Plugin for ReadRonDirectory<R> {
    fn build(&self, app: &mut App) {
        app.insert_resource(RonDirectories::<R>::new(&self.directory))
            .add_systems(
                Update,
                read_ron_files_from_directory::<R>.in_set(RonSystems::Read),
            );
    }
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> ReadRonDirectory<R> {
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: directory.into(),
            _marker: PhantomData,
        }
    }
}

#[derive(Resource)]
struct RonDirectories<R: Ingest + DeserializeOwned + Sync + Send + 'static> {
    directories: Vec<String>,
    _marker: PhantomData<R>,
}

impl<R: Ingest + DeserializeOwned + Sync + Send + 'static> RonDirectories<R> {
    fn new(directory: &String) -> Self {
        Self {
            directories: vec![directory.clone()],
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
