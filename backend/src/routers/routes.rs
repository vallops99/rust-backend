use axum::{Router, routing::get};

use crate::handlers::{root::root, posts::get_posts, posts::post_posts};

pub(crate) fn mount() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts).post(post_posts))
}
