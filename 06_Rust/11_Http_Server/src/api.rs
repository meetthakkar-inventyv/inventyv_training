use crate::{SharedState, handler::save_projects, model::*};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use std::time::Instant;
use std::thread;
use chrono::Local;

//
// ---------------- LOGGING HELPERS ----------------
//

fn log_start(endpoint: &str) -> Instant {
    let now = Local::now();

    println!("\n==================================================");
    println!("EXECUTING  : {}", endpoint);
    println!("THREAD     : {:?}", thread::current().id());
    println!("START TIME : {}", now.format("%H:%M:%S"));
    println!("==================================================");

    Instant::now()
}

fn log_end(endpoint: &str, start: Instant) {
    println!("--------------------------------------------------");
    println!("COMPLETED  : {}", endpoint);
    println!("THREAD     : {:?}", thread::current().id());
    println!("DURATION   : {:.2?}", start.elapsed());
    println!("--------------------------------------------------\n");
}

//
// ---------------- PROJECT CRUD ----------------
//

pub async fn get_projects(State(state): State<SharedState>) -> Json<Vec<Project>> {
    let start = log_start("GET /projects");

    let projects = state.read().await;

    log_end("GET /projects", start);

    Json(projects.clone())
}

pub async fn get_project(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Project>, StatusCode> {

    let endpoint = format!("GET /projects/{}", id);
    let start = log_start(&endpoint);

    let projects = state.read().await;

    let result = projects
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND);

    log_end(&endpoint, start);

    result
}

pub async fn create_project(
    State(state): State<SharedState>,
    Json(mut project): Json<Project>,
) -> (StatusCode, Json<Project>) {

    let start = log_start("POST /projects");

    project.id = Uuid::new_v4().to_string();
    project.tasks = vec![];

    let mut projects = state.write().await;
    projects.push(project.clone());

    save_projects(projects.clone()).await;

    log_end("POST /projects", start);

    (StatusCode::CREATED, Json(project))
}

pub async fn update_project(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(updated): Json<Project>,
) -> StatusCode {

    let endpoint = format!("PUT /projects/{}", id);
    let start = log_start(&endpoint);

    let mut projects = state.write().await;

    let status = if let Some(project) = projects.iter_mut().find(|p| p.id == id) {
        project.name = updated.name;
        project.description = updated.description;

        save_projects(projects.clone()).await;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };

    log_end(&endpoint, start);

    status
}

pub async fn delete_project(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {

    let endpoint = format!("DELETE /projects/{}", id);
    let start = log_start(&endpoint);

    let mut projects = state.write().await;

    let len_before = projects.len();
    projects.retain(|p| p.id != id);

    let status = if projects.len() != len_before {
        save_projects(projects.clone()).await;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };

    log_end(&endpoint, start);

    status
}

//
// ---------------- TASK CRUD ----------------
//

pub async fn get_tasks(
    Path(project_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Task>>, StatusCode> {

    let endpoint = format!("GET /projects/{}/tasks", project_id);
    let start = log_start(&endpoint);

    let projects = state.read().await;

    let result = projects
        .iter()
        .find(|p| p.id == project_id)
        .map(|p| Json(p.tasks.clone()))
        .ok_or(StatusCode::NOT_FOUND);

    log_end(&endpoint, start);

    result
}

pub async fn get_task(
    Path((project_id, task_id)): Path<(String, String)>,
    State(state): State<SharedState>,
) -> Result<Json<Task>, StatusCode> {

    let endpoint = format!("GET /projects/{}/tasks/{}", project_id, task_id);
    let start = log_start(&endpoint);

    let projects = state.read().await;

    let result = if let Some(project) = projects.iter().find(|p| p.id == project_id) {
        project
            .tasks
            .iter()
            .find(|t| t.id == task_id)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    } else {
        Err(StatusCode::NOT_FOUND)
    };

    log_end(&endpoint, start);

    result
}

pub async fn add_task(
    Path(project_id): Path<String>,
    State(state): State<SharedState>,
    Json(mut task): Json<Task>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {

    let endpoint = format!("POST /projects/{}/tasks", project_id);
    let start = log_start(&endpoint);

    task.id = Uuid::new_v4().to_string();

    let mut projects = state.write().await;

    let result = if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        project.tasks.push(task.clone());
        save_projects(projects.clone()).await;
        Ok((StatusCode::CREATED, Json(task)))
    } else {
        Err(StatusCode::NOT_FOUND)
    };

    log_end(&endpoint, start);

    result
}

pub async fn update_task(
    Path((project_id, task_id)): Path<(String, String)>,
    State(state): State<SharedState>,
    Json(updated): Json<Task>,
) -> StatusCode {

    let endpoint = format!("PUT /projects/{}/tasks/{}", project_id, task_id);
    let start = log_start(&endpoint);

    let mut projects = state.write().await;

    let status = if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        if let Some(task) = project.tasks.iter_mut().find(|t| t.id == task_id) {
            task.title = updated.title;
            task.completed = updated.completed;
            save_projects(projects.clone()).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    };

    log_end(&endpoint, start);

    status
}

pub async fn delete_task(
    Path((project_id, task_id)): Path<(String, String)>,
    State(state): State<SharedState>,
) -> StatusCode {

    let endpoint = format!("DELETE /projects/{}/tasks/{}", project_id, task_id);
    let start = log_start(&endpoint);

    let mut projects = state.write().await;

    let status = if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        let len_before = project.tasks.len();
        project.tasks.retain(|t| t.id != task_id);

        if project.tasks.len() != len_before {
            save_projects(projects.clone()).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    };

    log_end(&endpoint, start);

    status
}