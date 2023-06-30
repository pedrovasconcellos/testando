mod helloworld;
use warp::{Filter, Rejection, Reply};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Message {
    text: String,
}

#[tokio::main]
async fn main() {
    let get_home = warp::path::end()
        .map(|| helloworld::hello_world::get_hello_world());

    let get_route = warp::path("endpoint-get")
        .and(warp::get())
        .and_then(handle_get);

    let post_route = warp::path("endpoint-post")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_post);

    let routes = 
        get_home.or(get_route).or(post_route);

    println!("Server started at localhost:8080");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn handle_get() -> Result<impl Reply, Rejection> {
    let message = Message {
        text: "Hello, GET!".to_string(),
    };
    Ok(warp::reply::json(&message))
}

async fn handle_post(message: Message) -> Result<impl Reply, Rejection> {
    let success_msg = format!("Received POST message: {}", message.text);
    let response = warp::reply::with_status(success_msg, warp::http::StatusCode::OK);
    Ok(response)
} //CURL: curl -X POST -H "Content-Type: application/json" -d '{"text": "Hello, Server!"}' http://localhost:8080/endpoint-post