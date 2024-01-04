// allow unused imports while developing
#![allow(unused_imports)]

mod app;
mod components;

use gloo::console::log;
use wasm_bindgen_futures::js_sys::Math::log;
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

use components::group::{navbar::NavBar, login::Login};

#[derive(Serialize, Deserialize)]
struct Error {
    err: String,
    age: i32,
}

#[function_component(App)]
pub fn app() -> Html {
    // let log_msg = Callback::from(|message: String| log!(message));
    log!("Loaded Yew!");
    html! {
    <>
    <NavBar />
    <Login />
    </>
    }
}

pub fn list_to_html<T: ToString>(list: &[T]) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item.to_string()}</li>}).collect()
}