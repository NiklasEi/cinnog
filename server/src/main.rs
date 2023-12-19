use app::{App, Person, TestResource};
use bevy_core::Name;
use data_layer::DataLayer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(TestResource("testing 123".to_owned()));
    data.spawn((Person, Name::new("valentin")));
    data.spawn((Person, Name::new("stephan")));

    data.build(App).await
}
