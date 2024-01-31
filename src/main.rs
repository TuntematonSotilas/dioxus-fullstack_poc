#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
	#[route("/home")]
	Home {},
	#[route("/")]
    Index {},
}

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> { }
    }
}

#[component]
fn Index(cx: Scope) -> Element {
    render! {
        h1 { "Index" }
        Link {
            to: Route::Home {},
            "Go to home"
        }
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Home" }
        Outlet::<Route> { }
    }
}
