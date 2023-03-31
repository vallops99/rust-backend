mod types;
mod routers;
mod handlers;

#[tokio::main]
async fn main() {
    let router = routers::mount();
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}