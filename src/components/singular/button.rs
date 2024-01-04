use yew::{prelude::*, props};

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps{
    pub name: String,
    pub color: String,
    pub align: Option<String>
}
#[function_component(Button)]
pub fn button(btn: &ButtonProps) -> Html {
    let class_name = format!("btn {}", &btn.color);
    html! {
    <div class={&btn.align}>
        <button class={class_name}>{ &btn.name }</button>
    </div>
    }
}
