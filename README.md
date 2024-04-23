# Cinnog

A static site generator using [Leptos] with [Bevy ECS] as a data layer

**There is an [example app] which is hosted at https://cinnog.netlify.app/**

Cinnog uses the [island mode] of Leptos. Normal Leptos components are static and will be served as HTML. Only islands will be compiled to WASM and render client side.

## The Data layer

[Bevy ECS] is used as data layer while the static site generation is handled by Leptos. The idea is similar to what Gatsby does with GraphQL using a Bevy ECS World as an in-memory database. The API of Bevy ECS is very nice to work with as a user. It removes any need of an extra syntax for data queries.

Users can fill the data layer with content from the file system, external APIs, or anywhere else. When all data is loaded and processed, Cinnog can build a given Leptos app and will supply the data layer in a context. Inside components, you can run [Systems][bevy_systems] against the data layer (think GraphQL query in Gatsby) and use [Resources][bevy_resources].

# MSRV

Since this project relies on Bevy, it has the same MSRV policy: latest stable Rust.

### Improvements

(not in any specific order)
- Bevy ECS and Leptos have some namespace clashes that would be helpful to resolve (ECS Component vs Leptos Component)
- Could we get rid of the `frontend` crate in user code?
- In Leptos components, it should be easy to get the "current entity". Going via the path parameters directly seems odd and involves multiple steps.
  - Maybe this just needs a well working pattern using contexts in user space?
  - Or some automatic mapping of path params and Entities + a context?
- Extend example with routes generated from ECS (should already be possible since the App component has access to the data layer)

## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[Bevy ECS]: https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs
[Leptos]: https://github.com/leptos-rs/leptos
[bevy_systems]: https://bevy-cheatbook.github.io/programming/systems.html?highlight=system#systems
[bevy_resources]: https://bevy-cheatbook.github.io/programming/res.html
[example app]: https://github.com/NiklasEi/cinnog_example
[island mode]: (https://book.leptos.dev/islands.html)
