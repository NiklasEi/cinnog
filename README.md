# Cinnog

A static site generator using Leptos

**You can find an example app at https://cinnog.netlify.app/**

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website. Some additional changes were required to integrate the ECS data layer into the router.

## Experimenting with Bevy ECS

[Bevy ECS][bevy_ecs] is used as a data layer. The idea is similar to what Gatsby does with GraphQL using a Bevy ECS World as the database.

When all data is loaded and processed, Cinnog can build a given Leptos app and will supply the data layer in a context. Currently, you can run [Systems][bevy_systems] against the data layer and use their return value (think GraphQL query in Gatsby) and use the value of [Resources][bevy_resources].

Currently, helper methods/systems are in work that cen, for example, load markdown files from certain directories and convert them to HTML.

### Improvements

(not in any specific order)
- Can we upstream changes to how Leptos handles static routes that would allow us get rid of the custom fork?
- Bevy ECS and Leptos have some namespace clashes that would be helpful to resolve (e.g. ECS Component vs Leptos Component)
- Could we get rid of the `frontend` crate in user code?
- Users should not have to wrap static param systems in Mutex/Box
- Loading and transforming files should be simple
  - This will need a bunch of helper systems and an easy way to integrate systems from third party crates
- In components, it would be good to somehow get the "current entity". Going via the path parameters directly seems odd and involves more steps than I would like.
  - Maybe this just needs a well working pattern using contexts in user space?
  - Or some mapping of path params and Entities + a context
- Extend example to include markdown -> HTML + fontmatter in components
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

[bevy_ecs]: https://crates.io/crates/bevy_ecs
[bevy_systems]: https://bevy-cheatbook.github.io/programming/systems.html?highlight=system#systems
[bevy_resources]: https://bevy-cheatbook.github.io/programming/res.html
