use yew::prelude::*;


#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
    <>
    <section class="h-100">
    <div class="container h-100">
       <div class="row justify-content-sm-center h-100">
          <div class="col-xxl-4 col-xl-5 col-lg-5 col-md-7 col-sm-9">
             <div class="text-center my-5">
                <img width="100" height="100" src="https://img.icons8.com/ios-filled/100/login-rounded-right.png" alt="login-rounded-right"/>
             </div>
             <div class="card shadow-lg">
                <div class="card-body p-5">
                   <h1 class="fs-4 card-title fw-bold mb-4 text-center">{"Login"}</h1>
                   <form>
                    </form>
                   <div class="card-footer py-3 border-0" style="margin:5px;">
                      <div class="text-center">
                        {"Dont have an account?"}
                    </div>
                   </div>
                </div>
             </div>
          </div>
       </div>
    </div>
 </section>
    </>
    }
}