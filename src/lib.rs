use cfg_if::cfg_if;
pub mod app;
pub mod common;
pub mod components;
pub mod error_template;
pub mod fileserv;

cfg_if! { if #[cfg(feature = "hydrate")] {
  use leptos::*;
  use wasm_bindgen::prelude::wasm_bindgen;
  use crate::app::App;

  #[wasm_bindgen]
  pub fn hydrate() {
    common::tracing::init().unwrap();

    leptos::mount_to_body(move |cx| {
      view! { cx, <App/> }
    });
  }
}}
