use crate::{PersonId, PersonName, SiteName};
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
                    .map(|(id, person)| {
                        view! {
                            <li>
                                <a href=format!("/person/{}", id)>{person}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn get_persons(people: Query<(&PersonId, &PersonName)>) -> Vec<(String, String)> {
    people
        .iter()
        .map(|(id, person)| (id.0.clone(), person.0.clone()))
        .collect()
}
