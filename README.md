# dioxus_fullstack_poc
A POC of Dioxus fullstack

dioxus-fullstack : allows you to initially render static HTML on the server and then update that HTML from the client with WebAssembly

## Doc

https://dioxuslabs.com/learn/0.4/getting_started/fullstack


## Install / check required tools
Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [Doxius CLI](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)

## Run 
    dx build --features web
    cargo run --features ssr --release


## Hot Reload
    dx build --features web
    dx serve --features ssr --hot-reload --platform desktop