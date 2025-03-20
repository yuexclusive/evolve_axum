mod components;
mod message;
use components::chatbox::ChatBox;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {<ChatBox/>}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
