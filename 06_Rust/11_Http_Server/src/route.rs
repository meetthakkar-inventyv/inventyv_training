use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::api::*;

pub fn app_routes() -> Router<crate::SharedState> {
    Router::new()

        // PROJECT ROUTES
        .route("/projects", get(get_projects).post(create_project))
        .route("/projects/:id", get(get_project).put(update_project).delete(delete_project))

        // TASK ROUTES
        .route("/projects/:id/tasks", get(get_tasks).post(add_task))
        .route("/projects/:id/tasks/:task_id",
            get(get_task)
            .put(update_task)
            .delete(delete_task)
        )
}