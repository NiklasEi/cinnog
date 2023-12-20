use app::{App, Person, SiteName};
use bevy_core::Name;
use data_layer::DataLayer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));
    data.spawn((Person, Name::new("valentin")));
    data.spawn((Person, Name::new("stephan")));
    data.spawn((Person, Name::new("paul")));

    data.build(App).await
}
