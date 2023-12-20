# Testing Leptos as a static site generator (SSG)

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website.

`cargo make serve` serves the div directory with watch mode and hot-reload enabled.
`cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
`cargo make fmt` formats with `rustfmt` and `leptosfmt`.

## Experimenting with Bevy ECS

Bevy ECS is used in an attempt to add a data layer to Leptos as a static site generator.
The idea would be similar to what Gatsby does with Graphql, but using a Bevy ECS World as database.

The current data layer code is very minimal and can be found in the `data_layer` member of this workspace. In server, a new data layer is constructed and filled with soem test data. In a more complete project there would be helper methods/systems to e.g. load markdown files from certain directories and convert them to HTML.

When all data is loaded and processed, the data layer can build a given Leptos app and will supply itself in a context. Currently, you can run [Systems][bevy_systems] against the data layer and use their return value (think GraphQL query in Gatsby) and use the value of [Resources][bevy_resources].


[bevy_systems]: https://bevy-cheatbook.github.io/programming/systems.html?highlight=system#systems
[bevy_resources]: https://bevy-cheatbook.github.io/programming/res.html