use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TextFieldProps {
    pub label: String,
    pub placeholder: Option<String>,
    pub input_type: Option<String>,
    pub value: String,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    html! {
        <>
        <div class="mb-3">
            <label for="user-password" class="form-label">{ &props.label }</label>
            <input type={ props.input_type.clone() } class="form-control" placeholder={props.placeholder.clone()} value={ props.value.clone() } />
        </div>
        </>
    }
}
