use crate::{DataLayer, Ingest};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, Resource};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::system::Res;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::de::DeserializeOwned;
use std::fs::read_to_string;
use std::marker::PhantomData;
use std::path::Path;
use std::{fs, io};

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MarkdownSystems {
    Read,
    Convert,
}

struct ReadMarkdown<M: Ingest + DeserializeOwned + Sync + Send + 'static> {
    _marker: PhantomData<M>,
}

impl<M: Ingest + DeserializeOwned + Sync + Send + 'static> Plugin for ReadMarkdown<M> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            read_markdown_from_directories::<M>.in_set(MarkdownSystems::Read),
        );
    }
}

impl<M: Ingest + DeserializeOwned + Sync + Send + 'static> ReadMarkdown<M> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub trait MarkdownDataLayer {
    fn add_markdown_directory<M: Ingest + DeserializeOwned + Sync + Send + 'static>(
        &mut self,
        directory: impl Into<String>,
    ) -> &mut Self;
}

impl MarkdownDataLayer for DataLayer {
    fn add_markdown_directory<M: Ingest + DeserializeOwned + Sync + Send + 'static>(
        &mut self,
        directory: impl Into<String>,
    ) -> &mut Self {
        self.app.init_resource::<MarkdownDirectories<M>>();
        if !self.app.is_plugin_added::<ReadMarkdown<M>>() {
            self.add_plugins(ReadMarkdown::<M>::new());
        }

        let mut directories = self.app.world.resource_mut::<MarkdownDirectories<M>>();
        directories.directories.push(directory.into());

        self
    }
}

#[derive(Resource)]
struct MarkdownDirectories<M: Ingest + DeserializeOwned + Sync + Send + 'static> {
    directories: Vec<String>,
    _marker: PhantomData<M>,
}

impl<M: Ingest + DeserializeOwned + Sync + Send + 'static> Default for MarkdownDirectories<M> {
    fn default() -> Self {
        Self {
            directories: vec![],
            _marker: PhantomData,
        }
    }
}

fn read_markdown_from_directories<
    FrontMatter: Ingest + DeserializeOwned + Sync + Send + 'static,
>(
    mut commands: Commands,
    directories: Res<MarkdownDirectories<FrontMatter>>,
) {
    fn read_from_dir<FrontMatter: Ingest + DeserializeOwned + Sync + Send + 'static>(
        path: &Path,
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
    for directory in &directories.directories {
        let path = Path::new(directory);
        read_from_dir::<FrontMatter>(path, &mut commands)
            .unwrap_or_else(|e| panic!("Failed to read files from {}: {:?}", directory, e));
    }
}

fn read_markdown<FrontMatter: Ingest + DeserializeOwned + Sync + Send + 'static>(
    path: &Path,
    commands: &mut Commands,
) -> io::Result<Entity> {
    let file = read_to_string(path)?;
    let matter = Matter::<YAML>::new();
    let markdown = matter.parse(&file);
    let mut file = commands.spawn(());
    if let Some(front_matter) = markdown.data {
        let parsed_front_matter = front_matter.deserialize::<FrontMatter>()?;
        parsed_front_matter.ingest_path(&mut file, path);
        parsed_front_matter.ingest(&mut file);
    }
    file.insert(MarkdownBody(markdown.content));
    Ok(file.id())
}

#[derive(Component, Clone)]
pub struct MarkdownBody(pub String);

pub struct ConvertMarkdownToHtml;

impl Plugin for ConvertMarkdownToHtml {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (MarkdownSystems::Read, MarkdownSystems::Convert).chain(),
        )
        .add_systems(
            Update,
            convert_markdown_to_html.in_set(MarkdownSystems::Convert),
        );
    }
}

fn convert_markdown_to_html(markdown: Query<(Entity, &MarkdownBody)>, mut commands: Commands) {
    for (file, MarkdownBody(markdown)) in &markdown {
        let parser = Parser::new_ext(markdown, Options::all());
        let mut html = String::new();
        html::push_html(&mut html, parser);
        commands.entity(file).insert(Html(html));
    }
}

#[derive(Component, Clone)]
pub struct Html(pub String);
