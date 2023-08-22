use crate::components::counter::Counter;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

fn router_fallback(cx: Scope) -> View {
  let mut outside_errors = Errors::default();
  outside_errors.insert_with_default_key(AppError::NotFound);

  view! { cx,
    <ErrorTemplate outside_errors/>
  }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);

  view! {
    cx,
    <Title text="Test"/>

    <Router fallback=router_fallback>
      <Routes>
        <Route
          path=""
          ssr=SsrMode::OutOfOrder
          view=|cx| view! { cx, <Counter initial_value=1 step=1/> }
        />
      </Routes>
    </Router>
  }
}
