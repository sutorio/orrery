use axum::routing::get;
use axum::Router;

use crate::AppContext;

pub fn construct_routes(app_context: AppContext) -> Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(app_context)
}
