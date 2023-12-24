use crate::ToBundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In};
use bevy_ecs::system::Query;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::serde::de::DeserializeOwned;
use pulldown_cmark::{html, Options, Parser};
use std::fs::{read_to_string, File};
use std::{fs, io};

pub fn read_markdown_from_directory<'a, FrontMatter: ToBundle + DeserializeOwned>(
    In(path): In<&'a str>,
    mut commands: Commands,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = read_to_string(path.clone())?;
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(&file);
            let mut file = commands.spawn(());
            if let Some(front_matter) = markdown.data {
                file.insert(
                    front_matter
                        .deserialize::<FrontMatter>()?
                        .to_bundle_with_path(&path),
                );
            }
            file.insert(MarkdownBody(markdown.content));
        }
    }
    Ok(())
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

#[derive(Component, Clone)]
pub struct MarkdownBody(pub String);

#[derive(Component, Clone)]
pub struct Html(pub String);

pub fn convert_markdown_to_html(markdown: Query<(Entity, &MarkdownBody)>, mut commands: Commands) {
    for (file, MarkdownBody(markdown)) in &markdown {
        let parser = Parser::new_ext(&markdown, Options::all());
        let mut html = String::new();
        html::push_html(&mut html, parser);
        commands.entity(file).insert(Html(html));
    }
}
