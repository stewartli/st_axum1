use axum::{Json, Router, routing::get};
use std::sync::{Arc, Mutex};

mod job;
mod route;

use crate::job::Job;
use crate::route::{handle_job, handle_job_name, route_home};

#[tokio::main]
pub async fn start_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8090")
        .await
        .unwrap();

    let job1 = Job {
        name: "hello".to_string(),
        freq: "monthly".to_string(),
        done_at: "today".to_string(),
        output: true,
    };
    let mydb = Arc::new(Mutex::new(vec![Job::default(), Job::default(), job1]));

    let app = Router::new()
        .route("/job", get(async || Json(Job::default())))
        .route("/all", get(handle_job))
        // http://localhost:8090/all/hello
        .route("/all/{name}", get(handle_job_name))
        .with_state(mydb)
        // position order due to db
        .merge(route_home("/"));

    axum::serve(listener, app).await.unwrap();
}
