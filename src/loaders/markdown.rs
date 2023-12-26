use crate::Ingest;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, In, Query};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::de::DeserializeOwned;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_markdown_from_directory<'a, FrontMatter: Ingest + DeserializeOwned>(
    In(path): In<&'a str>,
    mut commands: Commands,
) -> io::Result<Vec<Entity>> {
    fn read_from_dir<FrontMatter: Ingest + DeserializeOwned>(
        path: &PathBuf,
        commands: &mut Commands,
    ) -> io::Result<Vec<Entity>> {
        let mut files = vec![];

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(read_markdown::<FrontMatter>(&path, commands)?);
            } else if path.is_dir() {
                files.append(&mut read_from_dir::<FrontMatter>(&path, commands)?)
            }
        }
        Ok(files)
    }
    let path = path.into();
    Ok(read_from_dir::<FrontMatter>(&path, &mut commands)?)
}

fn read_markdown<'a, FrontMatter: Ingest + DeserializeOwned>(
    path: &PathBuf,
    commands: &mut Commands,
) -> io::Result<Entity> {
    let file = read_to_string(path.clone())?;
    let matter = Matter::<YAML>::new();
    let markdown = matter.parse(&file);
    let mut file = commands.spawn(());
    if let Some(front_matter) = markdown.data {
        let parsed_front_matter = front_matter.deserialize::<FrontMatter>()?;
        parsed_front_matter.ingest_with_path(&mut file, &path);
    }
    file.insert(MarkdownBody(markdown.content));
    Ok(file.id())
}

#[derive(Component, Clone)]
pub struct MarkdownBody(pub String);

pub fn convert_markdown_to_html(markdown: Query<(Entity, &MarkdownBody)>, mut commands: Commands) {
    for (file, MarkdownBody(markdown)) in &markdown {
        let parser = Parser::new_ext(&markdown, Options::all());
        let mut html = String::new();
        html::push_html(&mut html, parser);
        commands.entity(file).insert(Html(html));
    }
}

#[derive(Component, Clone)]
pub struct Html(pub String);
