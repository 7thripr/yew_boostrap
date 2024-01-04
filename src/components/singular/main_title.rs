use yew::{prelude::*, props};

#[derive(Properties, Clone, PartialEq)]
pub struct TitleProps{
    pub title: String,
    pub on_load: Callback<String>,
}

#[function_component(MainTitle)]
pub fn main_title(title: &TitleProps) -> Html {
    title.on_load.emit("Title Loaded".to_owned());
    html! {
        <h1 class="text-center">{ &title.title }</h1>
    }
}