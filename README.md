# Testing Leptos as a static site generator (SSG)

**The example App is deployed on Netlify: https://ecs-leptos-ssg.netlify.app/**

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website.

- `cargo make serve` serves the App with watch mode and hot-reload enabled.
- `cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
- `cargo make fmt` formats with `rustfmt` and `leptosfmt`.
- `cargo make e2e` runs the end-to-end tests from the `end2end` directory.

## Experimenting with Bevy ECS

Bevy ECS is used in an attempt to add a data layer to Leptos as a static site generator. The idea is similar to what Gatsby does with GraphQL using a Bevy ECS World as the database.

The current data layer code is very minimal and can be found in the `data_layer` member of this workspace. In `generator`, a new data layer is constructed and filled with example data.

When all data is loaded and processed, the data layer can build a given Leptos app and will supply itself in a context. Currently, you can run [Systems][bevy_systems] against the data layer and use their return value (think GraphQL query in Gatsby) and use the value of [Resources][bevy_resources].

In a more complete project, there would be helper methods/systems to e.g. load markdown files from certain directories and convert them to HTML in `generator`. In this potential future, `data_layer` might be a library with a proper name and re-export `leptos` and `bevy_ecs` for simpler setup.

### Improvements

(not in any specific order)
- Can we upstream changes to how Leptos handles static routes that would allow us get rid of the custom fork?
- Bevy ECS and Leptos have some namespace clashes that would be helpful to resolve (e.g. ECS Component vs Leptos Component)
- Re-evaluate if more bevy crates would make sense (`bevy_app`, `bevy_assets`)
- Publish data layer and see if we could simplify the structure of user code
  - Could we get rid of the `frontend` crate in user code?
  - The example app should either be a separate repository or an example in the `data_layer` library
- Users should not have to wrap static param systems in Mutex/Box
- Loading and transforming files should be simple
  - This will need a bunch of helper systems and an easy way to integrate systems from third party crates
- In components, it would be good to somehow get the "current entity". Going via the path parameters directly seems odd and involves more steps than I would like.
  - Maybe this just needs a well working pattern using contexts in user space?
  - Or some mapping of path params and Entities + a context
- Extend example to include markdown -> HTML + fontmatter in components
- Extend example with routes generated from ECS (should already be possible since the App component has access to the data layer)


[bevy_systems]: https://bevy-cheatbook.github.io/programming/systems.html?highlight=system#systems
[bevy_resources]: https://bevy-cheatbook.github.io/programming/res.html
