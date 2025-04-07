use dioxus::{
    html::textarea::rows,
    logger::tracing::{self, Level},
    prelude::*,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/chat")]
    Chat {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(Level::DEBUG).unwrap();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            // h1 { class: "text-3xl font-bold underline", "Hello world!" },
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

#[component]
pub fn Chat() -> Element {
    rsx! {
        div {
            class: "h-screen grid grid-cols-5 gap-2",
            div {
                class: "bg-gray-200 col-span-1",
            }
            div {
                class: "bg-yellow-50 col-span-4 relative",
                div {
                    class: "left-1/2 top-10 -translate-x-1/2 absolute bg-slate-50 w-3/4 h-4/6 p-3",
                    div {
                        class: " bg-green-200 w-1/2 -translate-x-1/2 absolute",
                        "hello"
                    }
                    p{"1"}
                    p{"1"}
                    p{"1"}
                }
                textarea {
                    rows: 5,
                    class: "rounded-2xl border absolute left-1/2 bottom-10 -translate-x-1/2 w-3/4 h-1/6 resize-none p-3 hover:shadow-lg focus:shadow-lg focus:outline-none",
                }
            }
        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "columns-2 border space-x-2 border-gray-200",
            div{
                class: "border-r border-gray-200",
                "Column 1"
            }
            div{"Column 2"}
        }
    }
}

// // #[component]
// // fn DogApp(props: DogAppProps) -> Element {
// //     unimplemented!();
// // }

// /// Shared navbar component.
// #[component]
// fn Navbar() -> Element {
//     rsx! {
//         div {
//             id: "navbar",
//             Link {
//                 to: Route::Home {},
//                 "Home"
//             }
//             Link {
//                 to: Route::Blog { id: 1 },
//                 "Blog"
//             }
//         }

//         Outlet::<Route> {}
//     }
// }
