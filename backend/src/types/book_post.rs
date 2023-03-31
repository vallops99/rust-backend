use axum::{response::{IntoResponse, Response}, Json};

pub struct BookPost<'a> {
    title: &'a str,
    body: &'a str
}

impl<'a> IntoResponse for BookPost<'a> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub struct BookPosts<'a>(Vec<BookPost<'a>>);

impl<'a> IntoResponse for BookPosts<'a> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}