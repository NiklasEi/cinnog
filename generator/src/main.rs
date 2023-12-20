use app::{Age, App, PersonId, PersonName, SiteName};
use bevy_ecs::prelude::*;
use data_layer::DataLayer;
use leptos::serde;
use std::fs::File;
use std::{fs, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

    data.run(read_people)?;

    data.build(App).await
}

fn read_people(mut commands: Commands) -> io::Result<()> {
    for entry in fs::read_dir("people")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let id = file_name.trim_end_matches(".ron").to_owned();
            let file = File::open(path)?;
            let person_data: PersonData =
                ron::de::from_reader(file).map_err(|error| io::Error::other(error))?;
            commands.spawn((
                PersonName(person_data.name),
                Age(person_data.age),
                PersonId(id),
            ));
        }
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct PersonData {
    name: String,
    age: u8,
}
