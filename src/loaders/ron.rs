use crate::Ingest;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::{fs, io};

pub fn read_ron_files_from_directory<'a, D: Ingest + DeserializeOwned>(
    In(path): In<&'a str>,
    mut commands: Commands,
) -> io::Result<Vec<Entity>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = File::open(path.clone())?;
            let data: D = ron::de::from_reader(file).map_err(|error| io::Error::other(error))?;
            let mut entity = commands.spawn(());
            data.ingest_with_path(&mut entity, &path);
            files.push(entity.id());
        }
    }
    Ok(files)
}
