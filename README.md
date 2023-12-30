# Cinnog

A static site generator using [Leptos]

**There is an [example app] which is hosted at https://cinnog.netlify.app/**

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website. Some additional changes were required to integrate the ECS data layer into the router.

## The Data layer

[Bevy ECS] is used in an attempt to add a data layer to Leptos as a static site generator. The idea is similar to what Gatsby does with GraphQL using a Bevy ECS World as an in-memory database. The API of Bevy ECS is very nice to work with as a user. It removes any need of an extra syntax for data queries.

Cinnog is quite minimal at the moment and very experimental. In `generator`, a new data layer is constructed and filled with example data from markdown and `ron` files.

When all data is loaded and processed, Cinnog can build a given Leptos app and will supply the data layer in a context. Inside components, you can run [Systems][bevy_systems] against the data layer (think GraphQL query in Gatsby) and use [Resources][bevy_resources].

### Improvements

(not in any specific order)
- Can we upstream changes to how Leptos handles static routes that would allow us get rid of the custom fork?
- Bevy ECS and Leptos have some namespace clashes that would be helpful to resolve (e.g. ECS Component vs Leptos Component)
- Could we get rid of the `frontend` crate in user code?
- Users should not have to wrap static param systems in Mutex/Box
- In components, it would be easy to somehow get the "current entity". Going via the path parameters directly seems odd and involves multiple steps.
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
[example app]: 
