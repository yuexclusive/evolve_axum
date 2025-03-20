use crate::message::Message;
use web_sys::{
    MouseEvent, console,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChatBoxProps {}

#[function_component(ChatBox)]
pub fn chat_box(props: &ChatBoxProps) -> Html {
    let mousemove = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(|e| {
        let rect = e
            .target()
            .expect("mouse event doesn't have a target")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("event target should be of type HtmlElement");
        let x = e.client_x() - rect.client_left();
        let y = e.client_y() - rect.client_width();
        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
    }));

    let mut elememt = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("id")
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>()
        .set_onmousemove(mousemove.as_ref().dyn_ref());

    html! {
    <p>{"hello world"}</p>
    }
}
