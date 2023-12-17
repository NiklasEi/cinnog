# Testing Leptos as a static site generator (SSG)

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website.

`cargo make serve` serves the div directory with watch mode and hot-reload enabled.
`cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
`cargo make fmt` formats with `rustfmt` and `leptosfmt`.
