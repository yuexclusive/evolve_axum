use dioxus::{
    html::{audio::src, textarea::rows},
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
const TEST_1_IMAGE: Asset = asset!("/assets/1.png");
const BUBBLE_CSS: Asset = asset!("/assets/bubble.css");

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
        document::Link {rel: "stylesheet", href: BUBBLE_CSS}
        div {
            class: "h-screen grid grid-cols-24 gap-0",
            div {
                class: "col-span-1 border-r bg-gray-900 border-gray-800",
            }
            div {
                class: "col-span-5 border-r bg-gray-950 border-gray-800 h-screen flex flex-col",
                div {
                    class: "h-20 w-full  border-gray-800 flex items-center justify-center",
                    input{ class:"bg-gray-800 border border-gray-800 hover:border-gray-500 w-2/3 h-6 rounded-md placeholder-gray-600 placeholder-opacity-80 pl-2", placeholder:"Search"}
                    input { 
                        class: "bg-gray-800 border border-gray-800 hover:border-gray-500 w-1/6 h-6 rounded-md ml-2",
                        type: "button", 
                        value: "+"
                    }
                }
                div {
                    class: "overflow-y-auto flex-grow",
                    for _ in 1..=20 {
                        div {
                            class: "w-full h-16 border-gray-800 flex items-center hover:bg-gray-700 cursor-pointer pl-3 pr-4",
                            img {
                                alt: "对方头像",
                                class: "rounded-md mr-1 size-10",
                                src: TEST_1_IMAGE
                            },
                            div {
                                class: "w-full",
                                div {
                                    class: "flex items-center mb-1",
                                    div {
                                        class: "text-gray-200",
                                        "对方昵称"
                                    }
                                    
                                    div {
                                        class: "text-gray-500 text-xs ml-auto",
                                        "2025/4/21 21:21"
                                    }
                                }
                                p {
                                    class: "text-gray-500 text-xs",
                                    "你发了个啥啥啥啥啥啥啥啥啥啥啥"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "bg-black col-span-18 relative h-screen flex-col flex",
                div {
                    class: "h-12 w-full"
                }
                div {
                    class: " border-gray-800 border-b border-t p-4 w-full h-4/6 overflow-y-auto",
                    for _ in 1..=20 {
                        div {
                            class: "flex items-start mb-5",
                            img {
                                alt: "对方头像",
                                class: "rounded-md mr-3 size-9",
                                src: TEST_1_IMAGE
                            },
                            div {
                                class: "p-2 rounded-lg relative chat-bubble-left",
                                p {
                                    class: "text-gray-900 max-w-xl",
                                    "how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today"
                                }
                            }
                        }
                        div {
                            class: "flex items-start justify-end mb-5",
                            div {
                                class: "p-2 rounded-lg relative chat-bubble-right",
                                p {
                                    class: "max-w-xl",
                                    "how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today how are you today"
                                }
                            },
                            img {
                                alt: "自己头像",
                                class: "rounded-md ml-3 size-9",
                                src: TEST_1_IMAGE
                            }
                        }
                    }


                }
                div {
                    class: "w-full flex-grow",
                    textarea {
                            rows: "5",
                            class: "h-full text-slate-300 bg-transparent caret-blue-600 bottom-0 w-full resize-none hover:shadow-lg focus:shadow-lg focus:outline-none p-3",
                        }
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
