use anyhow::Result;
use axum::response::IntoResponse;
use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::{routing::{post}, Router, extract::{State}};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use this_app::{app::{App}, common, fileserv};

    async fn shutdown_signal() {
      let ctrl_c = async {
        tokio::signal::ctrl_c()
          .await
          .expect("failed to install Ctrl+C handler");
      };

      #[cfg(unix)]
      let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
          .expect("failed to install signal handler")
          .recv()
          .await;
      };

      #[cfg(not(unix))]
      let terminate = std::future::pending::<()>();

      tokio::select! {
          _ = ctrl_c => {},
          _ = terminate => {},
      }
    }

    async fn leptos_routes_handler(
      State(options): State<LeptosOptions>,
      req: http::Request<axum::body::Body>
    ) -> axum::response::Response {
      let handler = leptos_axum::render_app_to_stream_with_context(
        options,
        move |_| {

        },
        |cx| view! { cx, <App/> }
      );
      handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() -> Result<()> {
      common::tracing::init().unwrap();
      // Setting get_configuration(None) means we'll be using cargo-leptos's env values
      // For deployment these variables are:
      // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
      // Alternately a file can be specified such as Some("Cargo.toml")
      // The file would need to be included with the executable when moved to deployment
      let conf = get_configuration(None).await.unwrap();
      let leptos_options = conf.leptos_options;
      let addr = leptos_options.site_addr;
      let routes = generate_route_list(|cx: Scope| {
        view! { cx, <App/> }
      })
      .await;

      let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes_with_handler(routes, axum::routing::get(leptos_routes_handler))
        .fallback(fileserv::file_and_error_handler)
        .with_state(leptos_options);

      axum::Server::bind(&addr)
       .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
          .await?;

      tracing::info!("listening on http://{}", &addr);

      Ok(())
    }
  } else  {
    pub fn main() {
      // no client-side main function
      // unless we want this to work with e.g., Trunk for a purely client-side app
      // see lib.rs for hydration function instead
    }
  }
}
