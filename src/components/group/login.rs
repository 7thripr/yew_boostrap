use yew::prelude::*;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

struct LoginFormData {
    username: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let onchange = Callback::from(|event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>().value();
        log!("onchange: ", input);
    });
    let onsubmit = Callback::from(|event: 	SubmitEvent| {
        let target = event.target().unwrap();
        log!("onsubmit: ", target);
    });

    html! {
    <>
    <section class="h-100">
   <div class="container h-100">
      <div class="row justify-content-sm-center h-100">
         <div class="col-xxl-3 col-xl-4 col-lg-4 col-md-6 col-sm-8">
            <div class="text-center my-5">
               <img width="100" height="100" src="https://img.icons8.com/ios-filled/100/login-rounded-right.png" alt="login-rounded-right"/>
            </div>
            <div class="card shadow-lg">
               <div class="card-body p-5">
                  <h1 class="fs-4 card-title fw-bold mb-4">{"Login"}</h1>
                  <form>
                     <div class="mb-3">
                        <label for="user-email" class="form-label">{"Email address"}</label>
                        <input type="email" class="form-control" id="user-email" placeholder="name@example.com" onchange={&onchange}/>
                     </div>
                     <div class="mb-3">
                        <div class="mb-2 w-100">
                           <label for="user-password" class="form-label">{"Password"}</label>
                           <input type="password" class="form-control" id="user-password" onchange={onchange}/>
                        </div>
                        <div class="text-center">
                           <button class="btn btn-primary ms-auto" onsubmit={onsubmit}>
                           {"Login"}
                           </button>
                        </div>
                     </div>
                  </form>
               </div>
            </div>
         </div>
      </div>
   </div>
</section>


    </>
    }
}