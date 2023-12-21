use crate::{PersonId, PersonName, SiteName};
use bevy_ecs::system::Query;
use data_layer::{expect_resource, run_system};
use leptos::{component, view, IntoView};

#[component]
pub fn Navigation() -> impl IntoView {
    let site_name = expect_resource::<SiteName>().0;
    let persons = run_system(get_people);
    view! {
        <div class="nav">
            <span>{site_name}</span>
            <ul class="people-links">
                {persons
                    .into_iter()
                    .map(|(id, person)| {
                        view! {
                            <li>
                                <a href=format!("/person/{}", id.0)>{person.0}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn get_people(people: Query<(&PersonId, &PersonName)>) -> Vec<(PersonId, PersonName)> {
    people
        .iter()
        .map(|(id, person)| (id.clone(), person.clone()))
        .collect()
}
