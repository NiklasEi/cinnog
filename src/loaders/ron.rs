use crate::Ingest;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::{fs, io};

pub fn read_ron_files_from_directory<D: Ingest + DeserializeOwned>(
    In(path): In<&str>,
    mut commands: Commands,
) -> io::Result<Vec<Entity>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = File::open(path.clone())?;
            let data: D = ron::de::from_reader(file).map_err(io::Error::other)?;
            let mut entity = commands.spawn(());
            data.ingest_path(&mut entity, &path);
            data.ingest(&mut entity);
            files.push(entity.id());
        }
    }
    Ok(files)
}
