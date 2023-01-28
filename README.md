# TODO

- Problem:
* A `Suspense` component is in render loop when I define


- [ ] Create action playground

- [ ] The mutable operation Insert/update/delete each should be action
      in the dispatch should pass the record payload

- [ ] The read operation (list and count) should use create_resources using version of actions

- [ ] Question:
      How get the result, if it works successfully ???
      Maybe I should have a signal
      Appears that should read the signal from value(), but should certify what is returns in the first moment

- [ ] Maybe to create a Table/Grid model, to manage pagination, count, crud, etc.

# Leptos study project Axum

It showcases Leptos' ability to create both a client-side rendered app, and a server side rendered app with hydration, in a single repository.

## Client Side Rendering

To run it as a Client Side App, you can issue `trunk serve --open` in the root. This will build the entire
app into one CSR bundle. Make sure you have trunk installed with `cargo install trunk`.

## Server Side Rendering with cargo-leptos

cargo-leptos is now the easiest and most featureful way to build server side rendered apps with hydration. It provides automatic recompilation of client and server code, wasm optimisation, CSS minification, and more! Check out more about it [here](https://github.com/akesson/cargo-leptos)

1. Install cargo-leptos

```bash
cargo install --locked cargo-leptos
```

2. Build the site in watch mode, recompiling on file changes

```bash
cargo leptos watch
```

3. When ready to deploy, run

```bash
cargo leptos build --release
```

## Server Side Rendering without cargo-leptos

To run it as a server side app with hydration, you'll need to have wasm-pack installed.

0. Edit the `[package.metadata.leptos]` section and set `site-root` to `"pkg"`. You'll also want to change the path of the `<StyleSheet / >` component in the root component to point towards the CSS file in the root. This tells leptos that the WASM/JS files generated by wasm-pack are available at `./pkg` and that the CSS files are no longer processed by cargo-leptos. Building to alternative folders is not supported at this time.
1. Install wasm-pack

```bash
cargo install wasm-pack
```

2. Build the Webassembly used to hydrate the HTML from the server

```bash
wasm-pack build --target=web --debug --no-default-features --features=hydrate
```

3. Run the server to serve the Webassembly, JS, and HTML

```bash
cargo run --no-default-features --features=ssr
```
