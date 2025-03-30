use std::process::exit;

use axum::{routing::{get, post}, Extension, Router};

use redis::RedisError;
use todo_mash::controllers::{home, todo_new};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let redis = match build_redis_client().await {
        Err(err) => {
            println!("err while building redis client: {err}");
            exit(0);
        },
        Ok(c) => c,
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/todos/new", post(todo_new::controller))
        .route("/", get(home::controller))
        .layer(Extension(redis))
        ;


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn build_redis_client() -> Result<redis::Client, RedisError>{
    Ok(redis::Client::open("redis://localhost:6379/")?)
}
