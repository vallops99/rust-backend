use axum::Json;

use crate::types::book_post::{BookPost, BookPosts};

pub async fn get_posts<'a>() -> BookPosts<'a> {
    let posts: BookPosts = BookPosts {
        Vec::from([
            BookPost {
                title: "Title1",
                body: "Body of Title1"
            },
            BookPost {
                title: "Title2",
                body: "Body of Title2"
            }
        ])
    };

    posts
}

#[axum_macros::debug_handler]
pub async fn post_posts<'a>(payload: BookPost<'a>) -> BookPost<'a> {
    let new_book_post: BookPost = payload;

    new_book_post
}
