use axum::{response::{Response, IntoResponse}, Json};

pub struct RootMsg<'a> {
    msg: &'a str
}

impl<'a> IntoResponse for RootMsg<'a> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}