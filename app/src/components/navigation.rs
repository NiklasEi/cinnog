use crate::{Person, SiteName};
use bevy_core::Name;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;
use data_layer::{expect_resource, run_system};
use leptos::{component, view, IntoView};

#[component]
pub fn Navigation() -> impl IntoView {
    let test = expect_resource::<SiteName>().0;
    let persons = run_system(get_persons);
    view! {
        <div class="nav">
            <span>{test}</span>
            <ul class="people-links">
                {persons
                    .into_iter()
                    .map(|person| {
                        view! {
                            <li>
                                <a href=format!("/person/{}", person)>{person}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn get_persons(people: Query<&Name, With<Person>>) -> Vec<String> {
    people.iter().map(|name| name.as_str().to_owned()).collect()
}
