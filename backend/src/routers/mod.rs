use axum::Router;

mod routes;

pub(crate) fn mount() -> Router {
    Router::new().merge(routes::mount())
}