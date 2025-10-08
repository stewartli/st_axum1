use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::job::Job;

// home route
async fn handle_home() -> impl IntoResponse {
    let res = "<h1>this is home page for</h1>";
    Html(res)
}

pub fn route_home(p: &str) -> Router {
    Router::new().route(p, get(handle_home))
}

// job route
type JobState = Arc<Mutex<Vec<Job>>>;

pub async fn handle_job(State(x): State<JobState>) -> impl IntoResponse {
    let res = HashMap::from([("total", x.lock().unwrap().len())]);
    Json(res)
}

pub async fn handle_job_name(
    Path(name): Path<String>,
    State(x): State<JobState>,
) -> impl IntoResponse {
    let res = x.lock().unwrap();
    if let Some(job) = res.iter().find(|j| j.name == name) {
        // impl trait from Result
        Ok(Json(job.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
