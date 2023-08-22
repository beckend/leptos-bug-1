use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::response::Response as AxumResponse;
    use axum::{
      body::{boxed, Body, BoxBody},
      extract::State,
      http::{Request, Response, StatusCode, Uri},
      response::IntoResponse,
    };
    use leptos::*;
    use tower_http::services::ServeDir;
    use crate::app::App;

    pub async fn file_and_error_handler(
      uri: Uri,
      State(options): State<LeptosOptions>,
      req: Request<Body>,
    ) -> AxumResponse {
      let mut response_serve = get_static_file(&uri, &options.site_root, &req).await.unwrap();

      if response_serve.status() == StatusCode::OK {
          let headers = response_serve.headers_mut();
          // 30 days
          headers.insert(http::header::CACHE_CONTROL, "public, max-age=2592000".parse().unwrap());
          response_serve.into_response()
      } else {
          let handler = leptos_axum::render_app_to_stream(options.to_owned(), move |cx| view!{cx, <App/>});
          handler(req).await.into_response()
      }
    }

    async fn get_static_file(uri: &Uri, root: &str, request: &Request<Body>) -> Result<Response<BoxBody>, (StatusCode, String)> {
      let mut request_builder = Request::builder()
        .uri(uri);

      if let Some(x) = request.headers().get("Accept-Encoding") {
        request_builder = request_builder.header("Accept-Encoding", x);
      }

      match ServeDir::new(root)
        .precompressed_zstd()
        .precompressed_br()
        .try_call(
          request_builder
          .body(())
          .unwrap()
        ).await {
          Ok(res) => {
            Ok(res.map(boxed))
          },
          Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Something went wrong: {err}"),
          )),
        }
    }
  }
}
