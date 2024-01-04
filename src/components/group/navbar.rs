use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
    <>
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
  <div class="container-fluid">
    <a class="navbar-brand" href="/">{"MaintainBharat"}</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarToggle" aria-controls="navbarToggle" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarToggle">
      <ul class="navbar-nav">
        <li class="nav-item">
          <a class="nav-link" href="#">{"File a Complaint"}</a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="#">{"Complaint Status"}</a>
        </li>
      </ul>
    </div>
  </div>
</nav>

    </>
    }
}