mod api;
mod handler;
mod model;
mod route;

use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::model::Project;

pub type SharedState = Arc<RwLock<Vec<Project>>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let projects = handler::load_projects().await;

    let state = Arc::new(RwLock::new(projects));
    let app = route::app_routes().with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}