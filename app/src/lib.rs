use bevy_core::Name;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;
use bevy_ecs::query::With;
use bevy_ecs::system::{IntoSystem, Query};
use data_layer::expect_resource;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::sync::{Arc, Mutex};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ecs_leptos_ssg.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=HomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/test/hallo"
                        view=Test
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/404"
                        view=NotFound
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/person/*any"
                        view=PersonalHomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(personal_static_params))),
                        )
                    />

                </Routes>
            </main>
        </Router>
    }
}

fn empty_static_params() -> StaticParamsMap {
    StaticParamsMap::default()
}

fn personal_static_params(people: Query<&Name, With<Person>>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "any".to_string(),
        people.iter().map(|name| name.as_str().to_owned()).collect(),
    );
    map
}

#[derive(PartialEq, Params, Clone)]
struct PersonRouteParams {
    any: String,
}

#[derive(Resource, Clone)]
pub struct TestResource(pub String);

#[derive(Component)]
pub struct Person;

#[component]
fn PersonalHomePage() -> impl IntoView {
    println!("ok in PersonalHomePage");
    let param = use_params::<PersonRouteParams>();

    view! {
        <h1>{param.get().unwrap().any}</h1>
        <HomePage/>
        <Test/>
    }
}

#[component]
fn Test() -> impl IntoView {
    let test = expect_resource::<TestResource>().0;
    println!("{test}");

    view! {
        <p>{test}</p>
        <Counter/>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let route = use_route();
    let test = expect_resource::<TestResource>().0;
    println!("{test}");

    view! {
        <Navigation/>
        <h1>
            "Welcome to Leptos! At "
            {route.params().get().0.get("any").unwrap_or(&"no any param".to_owned())}
        </h1>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <button on:click=on_click>"Click Me: " {count}</button> }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <Navigation/>
        <h1>"Not Found from Leptos"</h1>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let test = expect_resource::<TestResource>().0;
    println!("{test}");
    view! {
        <ul>
            <li>
                <a href="/person/stephan">stephan</a>
            </li>
            <li>
                <a href="/person/valentin">valentin</a>
            </li>
        </ul>
    }
}
