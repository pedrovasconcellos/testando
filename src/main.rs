use warp::Filter;
mod helloworld;

#[tokio::main]
async fn main() {
    let filter = warp::path::end()
        .map(|| helloworld::hello_world::get_hello_world());

    println!("Server started at localhost:8080");
    warp::serve(filter)
        .run(([127, 0, 0, 1], 8080))
        .await;
}