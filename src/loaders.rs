use crate::ToBundle;
use bevy_ecs::prelude::{Commands, In};
use leptos::serde::de::DeserializeOwned;
use std::fs::File;
use std::{fs, io};

pub fn read_markdown_from_directory(In(path): In<&'static str>) {
    println!("Reading markdown files in directory {}", path);
}

pub fn read_ron_files_from_directory<'a, D: ToBundle + DeserializeOwned>(
    In(path): In<&'a str>,
    mut commands: Commands,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = File::open(path.clone())?;
            let data: D = ron::de::from_reader(file).map_err(|error| io::Error::other(error))?;
            commands.spawn(data.to_bundle_with_path(&path));
        }
    }
    Ok(())
}
