use std::sync::{Arc, Mutex};

use axum::{Json, Router, routing::get};

mod job;
mod route;

use crate::job::Job;
use crate::route::{handle_job, handle_job_name, route_home};

#[tokio::main]
pub async fn start_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8090")
        .await
        .unwrap();

    let mydb = Arc::new(Mutex::new(vec![Job::default(), Job::default()]));

    let app = Router::new()
        .route("/job", get(async || Json(Job::default())))
        .route("/all/{name}", get(handle_job))
        .route("/all", get(handle_job_name))
        .with_state(mydb)
        // position order
        .merge(route_home("/"));

    axum::serve(listener, app).await.unwrap();
}
